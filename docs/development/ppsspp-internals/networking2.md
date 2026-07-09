------------------------------------------------------------------------------------------------------------------------------------------------------------------------

A self-written implementation of the HTTP server and a simple thread executor (HTTPServer.h/.cpp and HTTPHeaders.h/.cpp).


class NewThreadExecutor;
Its only API is "Run(func)". Nothing fancy, just spawns a new OS thread. The destructor joins the threads.


class Server;
You construct it from a pointer to NewThreadExecutor.
The state management API: "Run(port)", "RunSlice(timeout)", "Listen(port, reason, dns_type)", "Stop()".

There are some getters: "ListenerSocket()", "Port()", "LocalAddress()".

To actually configure the behavior of the server, you have to register the URL handlers.
"RegisterHandler(url, handler)", "SetFallbackHandler(handler)".
If the URL doesn't correspond to any of the handlers, a fallback handler is called.
There can be only one handler for a single URL, so registering another handler removes the old one.

TODO: `handlers_[std::string(url_path)] = handler;` -> `handlers_[std::string(url_path)] = std::move(handler);`... Or maybe even emplace.
TODO: `fallback_ = handler;` -> `fallback_ = std::move(handler);`


If you need to customize the server even further, you have to inherit from this class and
override the virtual "HandleRequest(server_request)".
By default, `HandleRequest` simply calls `HandleRequestDefault(server_request);` and
the implementations are expected to forward to `HandleRequestDefault` if they don't recognize the url.


No matter what, the server registers a "HandleListing" handler for the root path "/"
that reports the URLs for currently enabled handlers (which is essentially the list of known URLs).
The default fallback handler responds with error 404.

State management API:
"Listen(port, reason, dns_type)" sets up the listening socket. After that the server's actual lifetime starts. The `dns_type` is for picking the inet type.
`reason` is only used for error logging. Internally calls either `Listen4` or `Listen6`.
If the `dns_type` is `DNSType::ANY`, tries IPv6 first.

`Listen4(port, reason)` and `Listen6(port, ipv6_only, reason)` are almost identical. The logic is as follows:

Create a socket with either AF_INET or AF_INET6 family, create a correct sockaddr struct and init it, enable SO_REUSEADDR with setsockopt.
`Listen6` then, depending on the ipv6_only arg (which is for distinguishing the case where we have DNSType::ANY and the case where we have DNSType::IPV6),
configures IPV6_V6ONLY with setsockopt.

Then they both `bind` to the socketaddr and bail out on failure (closing the socket) with error logs. Then they make the socket non-blocking.
Next they both 'listen` and bail out on failure (closing the socket).
I don't understand the next part, but we call `getsockname` and overwrite our input arg `port` with whatever is now in the `server_addr`'s field if the result is zero.
Lastly, we init `localAddress_` with a call to `fd_util::GetLocalIP(listenerSock_)` and copy the `port` to `port_`. And info log.



"RunSlice(timeout)" is the function for running the server's code. It waits until the listening socket is ready or until timeout expires. If ready, it accepts and
dispatches a thread in its executor to handle the connection (method `HandleConnection`), then returns true. Otherwise it returns false and logs the error.
Note: it may run for (significantly) longer than timeout, but won't wait longer than that for a new connection to handle.

Note: timeout is measured in seconds. If it's not positive, PPSSPP uses a timeout of 86400 seconds... Which is 24 hours.
TODO: add a comment explaining the magic value?

"Close" closes the listening socket, which makes the subsequent calls to `RunSlice` immediately return false.

"Run(port)" is a seemingly unused routine that sets up the server with default settings and infinitely calls `RunSlice` with timeout 0.
For some reason the `reason` is "websocket" here.
```
// Runs forever, serving requests. If you want to do something else than serve pages, better put this on a thread.
Returns false if failed to start serving, never returns if successful.
```


The HTTPServer represents the requests in the form of the class ServerRequest, which uses the `RequestHeader` class and the sinks.

class RequestHeader;

This is used by the server to parse the client's HTTP header and hold the results.

enum Method {
	GET,
	HEAD,
	POST,
	UNSUPPORTED,
};
Yes, PPSSPP doesn't recognize anything else. Not that it needs to, I guess... Maybe eventually DELETE?

enum RequestType {
	SIMPLE, FULL,
};
This is not documented. TODO...
The actual meaning is surprising... One of `SIMPLE`'s meanings is "HTTP 0.9". And `FULL` means "HTTP 1.0+".
Okay, to be completely honest, the *exact* meaning is "single line"
(and that could be due to either the request being HTTP 0.9 or PPSSPP not moving onto the next line because the request is malformed).
By the way, almost nothing supports HTTP 0.9 nowadays... And PPSSPP is no exception! This will become apparent when you see how the `ok` field is evaluated.


Public fields: `ok`, `method`, `type`, `other` (apparently a map from string to string for other headers), `content_length`, `status`
and const char ptrs `referer`, `user_agent`, `resource` and `params`.

"RequestHeader()" - the empty constructor, sadly not marked as default.
The parsing and initialization happens in the method "ParseHeaders(input_sink)".
We read the input line by line (getting an empty line is the end marker).
If by the end we will have processed at least 2 lines and gotten a non-empty resource, we'll set `ok` to true.
So PPSSPP doesn't a

TODO: explain what is "ok" in a comment next to a field?

Each line is fed into "ParseHttpHeader(str_ptr)". Initially the field `first_header_` is set to true, which forces a special branch to be taken.

Branch 1)
In it we set `first_header_` to false, parse the first line, starting from the method. 
t's GET/POST/HEAD, anything else is unsupported and causes 405 and immediate returning of -1. The caller `ParseHeaders` doesn't check the return value!
Because the `type` wasn't modified, it remains `SIMPLE` and PPSSPP logs `Simple: Done parsing http request.` Because the number of lines is 1, it's not `ok`.
If the method is recognized, we try to parse the resource (the request URL without the queries). Failing to do this results in 400 and returning of -1.
Which, again causes the stuff mentioned above (the logs, the SIMPLE type and 
If there are any HTTP queries, they are saved as a contiguous string.
Now then, if the data after the queries contains a substring "HTTP/", we change the `type` to FULL. return 0.

Branch 2)
If the `first_header_` is false, we parse the line as a valid HTTP header. So we look for ":". Not found => 400 and bail out with -1.
If found, PPSSPP checks if it's either of "User-Agent", "Referer", "Content-Length". These special cases are handled immediately (PPSSPP saves the values).
Other headers are saved into unordered map "string -> string" `other` (the key is lowercased first).
TODO: `other.emplace(std::move(key_str), buffer);`?


TODO: Log::IO -> Log::HTTP?

Getters: "GetParamValue(param_name_ptr, out_value)" and "GetOther(name_ptr, out_value)".

"GetOther" is for accessing headers which are not "User-Agent", "Referer", "Content-Length".
Because the keys are lowercased on insert, passing in a non-lowercase string will find nothing. This function doesn't lowercase the argument before lookup.
TODO: write a comment.

"GetParamValue" is for accessing request params. It only works for `name=value` params.
If there's anything else (like "/resource/path?value"), crashes PPSSPP.
TODO: fix


Finally, time for

class ServerRequest;

It stores and manages the client socket `fd_` (from `accept`) and also `header_` (an object of class `RequestHeader`), `in_` and `out_` (`InputSink` and `OutputSink`).

"ServerRequest(fd)" - the constructor. It inits the sinks and invokes `header_.ParseHeaders`. If the `header_` is not `ok`, calls `Close`.
"Close()" - if there's a valid socket, calls `closesocket` and resets the field `fd_`.

The dtor also calls `Close`. Then it error logs if the input sink isn't empty and deletes it. Lastly it warn logs if the out sink isn't empty and deletes it.

This class has a lot of accessors:
"fd()" for `fd_`, and "In()", "Out()" for the sinks. Apparently there's a TODO for removing "fd()".
"IsOK()" for checking if `fd_` is bigger than 0.

"resource()" for reaching into `header_.resource`, "Method()" for reaching into `header_.method` and "Header()" for the `header_` itself.
Then there are 2 more "forwarders":
1) "GetParamValue(param_name_ptr, out_value)" which calls, obviously, `header_.GetParamValue`
2) "GetHeader(name_ptr, out_value)" which calls, not obviously, `header_.GetOther`.

The previous comments still stand: `GetParamValue` ends up being dangerous and `GetHeader` can't access "User-Agent", "Referer", "Content-Length".
Note: here there is a comment about lowercasing the header's name!


Now we only have "Write()", "WritePartial()" and "WriteHttpResponseHeader(version_str_ptr, status, size, mime_type, other_headers)".
These are supposedly the helpful routines for answering to the request. Well.

"WritePartial" simply flushes the out sink ("send all", blocking allowed) after asserting that fd is truthy.
"Write" simply calls `WritePartial` after asserting that fd is truthy and then calls `Close`. So it's a "flush and close" function.

"WriteHttpResponseHeader" writes a complete HTTP response header by using the output sink.
The first line is formatted with the supplied version and the status (http code).
The headers are:
1) "Server: PPSSPPServer v0.1" (wait, it's versioned???)
2) If `mime_type` is nullptr, "Content-Type: {DEFAULT_MIME_TYPE}" (that is, "text/html; charset=utf-8") and "Connection: close".
3) If `mime_type` is not nullptr and it's not equal to the char sequence "websocket", "Content-Type: {mime_type}" and "Connection: close".
4) If `size` is not negative, "Content-Length: {size}". So you can pass -1 to disable this header entirely.
5) If `other_headers` is not nullptr, it is inserted as is. So you gotta put the \r\n at the end of last header yourself.
Then the header is terminated by \r\n.

TODO: move a big ahh switch into a standalone function?
TODO: maybe rename local var `buffer` to something else? Even though you can see here that it's a sink and not a Buffer (or net::Buffer), the methods
are kinda similar... It kinda confuses me a bit... Just a nitpick.
Actually, it was just a Buffer until https://github.com/hrydgard/ppsspp/commit/41e7d3ecde9539c87d3371280217d6487c804bfa, but the name stayed.

Note: this class doens't have a lot of helper utils. Essentially only the `WriteHttpResponseHeader`...
The rest is just an aggregation of other classes. You have to use the out sink to write the answer and the in sink for reading the body of the request.


So, the `HandleConnection` method of `Server` constructs an instance of `ServerRequest` from the accepted sock,
warn logs if it's not ok (in that case we bail out) and calls `HandleRequest`. There are some TODOs here...
Well, the function finishes by calling `request.Write` to "flush and close".

`HandleRequestDefault` invokes the fallback handler (passing in the request) if the resource is "empty".
If not, it fetches a handler for the resource by doing a map lookup and invokes it (passing in the request).
And if not found, then again we fall back to the fallback handler.

TODO: remove the `std::bind`s from the handlers and the `executor->Run` command (replace with lambdas for clarity).

Sink pause.
------------------------------------------------------------------------------------------------------------------------------------------------------------------------

The server side manages the data sending/receiving through special buffered socket wrappers known as sinks (in Sinks.h/.cpp).

class InputSink;

It stores a socket fd and a ringbuffer as a group of fields (namely `buf_`, `valid_`, `read_` and `write_`).
```
// Circular buffer. read_ is the position to read from, write_ is the position to write to,
// valid_ is the number of valid bytes. valid_ can wrap around, so you might need to take
// two segments when reading and split when writing.
```
It's never said explicitly, but it would seem that there's a contract "valid_ <= BUFFER_SIZE". Which makes sense, because that's the buffer's capacity.
The ringbuffer is managed very carefully in the sink to make sure `write_` never overtakes `read_`.


"InputSink(sock_fd)" - the constructor makes the socket non blocking.
TODO: this is getting ridiculous... Here `sock_fd` is `size_t`. So we have `int`, `uintptr_t` and `size_t` in the codebase for socket handles.

Note: sorry, but the methods down below are not sorted based on their public/private specifiers.

"AccountFill(bytes_cnt)" - this is only used from inside `Fill` and it's supposed to update the sink's state after a call to `recv`
that returned `bytes_cnt`. Which could still be an error (negative). AccountFill accepts int to handle errors.
The only thing this does in case of error is "optionally error log, if the error is not EWOULDBLOCK and not EAGAIN, and return".
If the recv is successful, this advances the ringbuffer.

Implicit precondition: the number of bytes is supposed to be realistic (bytes_cnt <= BUFFER_SIZE).

Note: `if (write_ >= BUFFER_SIZE)` might even be excessive, because `write_` can't become bigger than BUFFER_SIZE.
Otherwise the call to recv would have overshot the buffer.
It could have become bigger than the buffer's size if we recved into the buffer with wrap around, but we never do that as of now.
So not saying it's something that needs to be changed, on the contrary, this is well-designed, just in case we start doing that in the future.



"Fill()" - a recv wrapper. It does 2 things... Firstly, it ignores "small" reads. Secondly, it recvs into the available space in the ringbuffer and calls `AccountFill`.
It doesn't always extract the whole message from the kernel because of the ringbuffer. The "small read" condition to allow the read is "BUFFER_SIZE - valid_ > PRESSURE".
So yeah, `Fill` can fail sometimes.

"Block()" - a horrible name. It waits for the socket to become ready (no more than 5 seconds; if timeout is reached, returns false) and calls `Fill`.
If no timeout, then returns true.

Note: Because `Fill` never reports if it actually read anything, this method also never reports if it actually read anything.
TODO: maybe "WaitAndFill"? Or something else...


"Discard()" - resets the ringbuffer.

"Empty()" - checks if `valid_` is zero.

"FindNewline()" - searches for a linebreak in the ringbuffer (returns the BUFFER_SIZE if not found).

"AccountDrain(bytes_cnt)" - updates the sink's state after some data has been "read" from it. Here the input type is size_t, it doesn't handle errors.
Implicit precondition: the number of bytes is supposed to be realistic (bytes_cnt <= BUFFER_SIZE).

Note: this one is used in all sorts of places, sometimes when the read spans for both chunks (standard and wrapped) => the ">=" check is necessary.


"ValidAmount()" - returns `valid_`.


String reading API:

"ReadLine(out_string)", "ReadLine()", "ReadLineWithEnding(out_string)", "ReadLineWithEnding()".

ReadLineWithEnding(out_string) and ReadLineWithEnding() return a string with linebreak at the end;
ReadLine(out_string) and ReadLine() return a string after trimming the last \n or \r\n.
The latter pair of methods is implemented in terms of former pair.

It all boils down to "ReadLineWithEnding(out_string)". First we search for the linebreak by calling `FindNewline`.
If found, we resize the `out_string`, fill it with data and call `AccountDrain`.

If not found... Oh boy. Then we call `Block` and call `FindNewline` again. And if it's still BUFFER_SIZE, we assume it's a timeout! Which makes no sense!
The comment " // Timed out." is misleading. If the author wanted to check for timeout, why didn't they check the return value of `Block`?
Furthermore... The first call can fail due to the "stuck" state of the sink where it doesn't wanna recv anymore (small read) and not because of a timeout.
And another thing: in that case calling `FindNewline` the second time makes a useless O(n) sweep over the ringbuffer, even though it won't find anything.


So yeah, all string reading functions that accept `out_string` may return false if the sink is stuck. The argumentless methods never check for the return value and
just give the caller an empty string if we're stuck. The caller wouldn't even notice. Yeah........................


TL;DR: something's kinda meh here, to say the least.


I think no one uses the "ReadLineWithEnding" API. `RequestHeader::ParseHeaders` uses the `ReadLine(out_string)` API to check for errors.
And `HandleMultipartPart` doesn't check for errors, so it could be theoretically wrong. TODO: cook an example.


Byte reading API:

"TakeAtMost(buf_ptr, bytes_cnt)" and "TakeExact(buf_ptr, bytes_cnt)" do what they say... The first one might even not read anything.


"TakeAtMost(buf_ptr, bytes_cnt)" - calls `Fill` and, if there's anything available, copies it to the output buffer and calls `AccountDrain`.
It returns the number of bytes that it managed to read (aka the available bytes).
If we're stuck, that means `Fill` would not do anything, but it's fine here, because being "stuck" means there's something in the buffer and we can return that.

Note: this function uses 1 call to `memcpy`, so if the ringbuffer is wrapped, it only returns the first part (until the end).

It can sometimes return 0 (no bytes have been read). It's when either you pass 0 for the `bytes_cnt` or there's nothing available after a call to `Fill`.
The latter can only happen if `recv` fails or there's just nothing for us (and in that case we have to wait for the segment to arrive).

"TakeExact(buf_ptr, bytes_cnt)" - uses `TakeAtMost` to read the exact number of bytes as requested. Returns whether the operation has been successful.
If bytes_cnt is zero, returns true immediately.
There is a loop that calls `TakeAtMost` repeatedly, updates the `buf_ptr` and `bytes_cnt` values after each read and checks if we're done.
If at any point `TakeAtMost` returns zero, that means either `recv` failed or there's just nothing for us...
So we call `Block` to wait until there's data... And `Block` calls `fd_util::WaitUntilReady`, followed by `Fill`. This helps if there's actually more data pending.
If `WaitUntilReady` returns false, we know it's a timeout (5 seconds) and bail out of the loop by returning false (operation failed).

This clearly doesn't check the possibility of a `recv` error. `WaitUntilReady` is just a select wrapper.
https://stackoverflow.com/questions/17817874/detecting-whenever-socket-disconnected-using-select:
```
When a socket closes it becomes "readable" but calling recv will return 0 bytes.
Using select you can tell when the socket can be read then when reading it if recv returns 0 then you know it has closed.
In non-blocking sockets, recv will return -1 and set errno to EWOULDBLOCK (or EAGAIN) if there is no data and the socket is not closed.
```
This scenario is not considered at all in the code. Therefore it's not surprising that a call to `TakeExact` may hang.

TODO: VERY carefully redesign at least some part of the InputSink to make it not hang.


"Skip(bytes_cnt)" - skip some bytes (as if reading into nothingness). Implemented in a similar fashion to `TakeExact`. And almost surely suffers from the same issue.
Like for real, if `bytes_cnt` is bigger than `valid_`, it calls `Block`. The if is literally the same as in `TakeExact`!
And it's also inside a loop that would only stop once everything is "read" or timed out...

TODO: VERY carefully redesign at least some part of the InputSink to make it not hang.


"TryFill()" - calls `Fill` and returns `!Empty()`. It's only used once, in the websocket's `WebSocketServer::Process`, to check for disconnect:
```
if (FD_ISSET(fd_, &read)) {
		if (in_->Empty() && !in_->TryFill()) {
```


"BufferParts()" - returns a pair of string views to the buffer parts (the second one is either empty or the wrapped one).
I'm not a fan of this name, because the "buffer" could be a verb ("please, buffer the parts").


"ReadBinaryUntilTerminator(out_buf, buf_size, terminator, out_did_read_terminator)" - reads a portion of bytes into the output buffer
until it finds a byte sequence `terminator`. Sets the bool out arg to whether the terminator was actually found. Returns the number of bytes actually read.
So the idea is to put this in a loop and break once the out bool arg is set to true.

If our needle's not found, leaves enough space in the ringbuffer just in case the needle's partially there (for the subsequent calls).

The search is performed by calling `SplitSearch` from StringUtils.h/.cpp, which I find to be a bit off.
It's only purpose is to search for a string in a ringbuffer, represented by 2 string views on its parts. If the needle is empty, assumes it's found in the first part.

It's never stated, but it would be logical to say that the first part is never empty, but the second part is often empty.
While I was investigating this, I've noticed that the condition `overlap > 0` is too weak, because once the overlap becomes too small,
the remaining chunk of the needle could become too big to fit in the second part.
|needle| - overlap has to be <= |part2|. Which means that overlap >= |needle| - |part2|. We can't really just drop that into the code, because the sizes are unsigned
and will underflow to 2^64 if the needle is smaller than part2, so instead I say we
TODO: `size_t minOverlap = /* 1 if needle is smaller, otherwise the difference */` and then `overlap >= minOverlap` or something like that.

That can save us from useless O(n^2) searches in the suffixes when it's obvious that the part2 won't be able to contain the remaining bytes.


Okay, back to `ReadBinaryUntilTerminator`... If the terminator is found, we `TakeExact` the bytes before it and `Skip` the terminator.
With cursed `_dbg_assert_(valid_ >= 0);` (`valid_` is unsigned, this is always true).
If not found, we read everything we can, but leave space for the terminator... But in which buffer? The ringbuffer or the output buffer?

I think there's a logic error here somewhere because...
There's no point of "leaving space" for the terminator in the output buffer if it's not gonna be copied to the output buffer. But that's exactly what the code does!
```
const s64 toRead = std::min((s64)valid_, (s64)bufSize - (s64)terminator.length());
TakeExact(dest, toRead);
```
By the way, that's ugly, a ternary or an if statement would've been better (also worth noting that TakeExact wants size_t and not s64).
Moreover, only the "not found" branch uses the function argument `buf_size`. Are we overshooting the output buffer?
I mean, we don't in the only place where it's used, because the output buffer very conveniently is defined as `char buffer[net::InputSink::BUFFER_SIZE];`.
But change that... And oh boy, oh boy.

So.
TODO: rethink this function. It's probably broken.


Note: because we call the hanging methods here, this method can also hang.



Now onto
class OutputSink;

The ringbuffer docs are only present in the InputSink's code.
When it comes to terminology, this sink passes the `buf_ + read_` pointer to `send`, so it "reads" our input and we "write" the data.

For some reason the BUFFER_SIZE here is defined normally, but the InputSink has some anonymous enum.


"OutputSink(sock_fd)" - the constructor makes the socket non blocking.
This follows the InputSink: `sock_fd` is `size_t`. So we have `int`, `uintptr_t` and `size_t` in the codebase for socket handles.

"AccountPush(bytes_cnt)" - just like InputSink's `AccountFill`, but doesn't check for errors (and accepts size_t),
because the pushed data here doesn't come from a call to `recv`.


"AccountDrain(bytes_cnt)" - just like InputSink's `AccountDrain`, but checks for errors (and accepts int), because here the data is passed to `send`.
After my refactor, InputSink's `AccountDrain` behaves just like this one in case of an error. But hrydgard forgot to update the logging category for the output sink.
TODO: IO -> Net

This is only used from inside `Drain` and `Flush`.
If the send is successful, this advances the ringbuffer.
Note: these accountants have the same preconditions as their cousins from InputSink (the number of bytes is supposed to be realistic).


"Empty()" - returns `valid_ == 0` 

"BytesRemaining()" - returns `valid_`. Just like InputSink's `ValidAmount`.

"Discard()" - like InputSink's one, resets the ringbuffer.


"Drain()" - like InputSink's `Fill`. A `send` wrapper. It also ignores "small" reads. Well, techniclaly, sends, but that's the terminology.
If the `valid_` is smaller than PRESSURE, nothing is sent. Secondly, it sends the available bytes from the ringbuffer and calls `AccountDrain`. 
It doesn't always send the whole message to the kernel because of the ringbuffer.

"Block()" - again, kinda bad name, imo. It uses `fd_util::WaitUntilReady` for waiting until the socket becomes writeable (with 5 seconds timeout) and
then calls `Drain()`.
TODO: think of a better name.

"Flush(allow_block)" - this method doesn't have any analogues in InputSink. It sends the whole message to the kernel by using a loop `while (valid_ > 0)`.
Even if the threshold is not reached yet. The `send` return value is passed to `AccountDrain`.
If the return value is negative, we return false. If the return value is zero... This branch reminds me of the same branch in InputSink's `TakeExact`:
that one calls `Block()` and returns false if it reports a timeout. This one also does this, but only if the argument `allow_block` is set to true.
If it's not set, we just immediately continue to the next iteration.


Note: both `Drain` and `Flush` have a weird section with a define:
```
        int bytes = send(fd_, buf_ + read_, avail, MSG_NOSIGNAL);
#if !PPSSPP_PLATFORM(WINDOWS)
		if (bytes == -1 && (socket_errno == EAGAIN || socket_errno == EWOULDBLOCK))
			bytes = 0;
#endif
```
It was added at https://github.com/hrydgard/ppsspp/commit/c7becb2c7512aaf37c85af62d43cdf94e6f5fdbd. But back then `AccountDrain` didn't check the error.
Can this be safely cut away? I'm pretty sure it can for `Drain`... And not sure about  `Flush` - that one might need a refactor after.


Note: the threshold means that `Drain` can fail sometimes. I don't know if it's important here, like it was in InputSink's `Fill`.

Writing API:
"PushAtMost(in_buf, bytes_cnt)" - it almost exactly mirrors the `TakeAtMost` from InputSink. But there's a small difference.
Namely, it has a "fast path for sending large chunked sends", added in https://github.com/hrydgard/ppsspp/commit/11c7c25b03efd0beb987a95da64d7deb996212a4
If there's nothing in the ringbuffer after a call to `Drain` and the requested size is bigger than the threshold, we try to send without copying the data to out buffer.
If that sends at least something, we're happy and return the number of bytes sent. Otherwise we proceed to copy the data to the ringbuffer and call `AccountPush`.

Note: I don't know if this is really a helpful optimization and whether a similar optimization can be applied to `TakeAtMost` if the buffer remains empty after `Fill`.
As in, recving directly into the user's buffer and returning if succeeded.

Note: this function uses 1 call to `memcpy`, so it only fills the first part (until the end) and doesn't wrap.

So yeah, this pushes some bytes into the buffer, maybe less than `bytes_cnt`.

It can sometimes return 0 (no bytes have been pushed). It's when either you pass 0 for the `bytes_cnt` or the buffer is full.
The latter can only happen if `send` fails (inside `Drain`).



"Push(in_buf, bytes_cnt)" - it exactly mirrors `TakeExact` from InputSink. And it can hang just like `TakeExact`.
The buffer becomes full, but we can't send anything, because we're disconnected (`send` fails, `fd::WaitUntilReady` immediately returns true). And we're inside a loop
that will keep running until everything is sent. There is no way out.

TODO: VERY carefully redesign at least some part of the OutputSink to make it not hang.


"Push(string)" - simply calls `Push(&string[0], string.length())`.


"PushCRLF(string)" - pushes the string and then adds "\r\n". It's unused. Otherwise someone would've noticed a typo (it actually sends 'r' instead of '\r').

Lastly, we have a "Printf(fmt_ptr, ...)", which for some reason is not annotated with `MSVC_FORMAT_PRINTF`. TODO?
This is a C variadic that uses vsnprintf to format the args into the ringbuffer. If we succeed, we call `AccountPush` and return true.
If the result is negative (some vsnprintf error, perhaps, wrong format string), we error log and return false.
TODO: fix logging levels here too.

If the result is positive, but bigger than the contiguous allowed space, that means that the output doesn't fit in the ringbuffer (could be due to wrap around).
Then we create a temp buffer on the stack and try again. If it's negative, we error log and return false.
If it fits in the temp buffer and is not zero bytes long, we oddly check if the last character in the buffer is '\0'.

This is odd, because the docs say that vsnprintf doesn't include the terminating null byte in its return value.
So we're essentially checking if the last user byte is formatted as '\0'. Which is not impossible (`Printf("%s%c", "Sup!", '\0')` ends up writing \0 twice).
And in that case we decrement `result`. So we cut off the user's byte. I don't think this was intended.
TODO: clarify.

Anyway, after the check and potential decrement, we `Push` the temp buffer's content and reset `result` to zero, because otherwise the following code
would have counted these bytes again inside `AccountPush`. Then we forward the return value of `Push` to the user. If it failed, we say we failed too.
Note: it's the very same `Push` that can hang. So we can also hang. Until `Push` is fixed, I mean.

But what if the formatted string doesn't fit in the temp buffer? Then we error log and return false.


TODO: empty format string when the buffer is full returns false and error logs "Not enough space to format output.". Which is kinda incorrect.

Note: The temp buffer used to be BUFFER_SIZE, but then hrydgard changed it to 4096 bytes. Does that affect the performance in a good way?
That's, like, 8 times smaller than the ringbuffer. And we use that when we run out of space in the ringbuffer. Is the math mathing?

TODO: there's a bug. We create a backup va_list "in case we have insufficient space" and never use it in the "no space" branch.
`result = vsnprintf(temp, sizeof(temp), fmt, args);` -> `result = vsnprintf(temp, sizeof(temp), fmt, backup);`


------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Back to `Server` and `ServerRequest`.
> You have to use the out sink to write the answer and the in sink for reading the body of the request.

The default 404 handler uses `WriteHttpResponseHeader` to write the HTTP header and uses `Out()->Push(payload)` to actually write the simple 404 response.
Verson is "1.0" (huh), code is 404 and the content size is payload's length.
TODO: STR_VIEW in the info log.


The listing handler also uses `WriteHttpResponseHeader`. Version is "1.0". Code is 200. No content length. Then it iterates over the handlers and
`Printf`s them into the out sink. With linebreaks.


Only the websocket server remains... And the webserver.

------------------------------------------------------------------------------------------------------------------------------------------------------------------------