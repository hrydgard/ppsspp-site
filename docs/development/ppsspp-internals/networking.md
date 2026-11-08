In Common/Net we have the generic network-related components. Mostly for HTTP.
Mostly coming from hrydgard's old webserver codebase. Some things here and there might be suboptimal, so to speak.

------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Common/Net/SocketCompat.h - that's the rather new (only 1.5 y.o.) interface for cross-platform socket-related operations. It has all the required #defines and #undefs
for the supported platforms. I hope that eventually the whole codebase will move to this API. For instance, here we have a define for `socket_errno` that
turns into a call to `WSAGetLastError` on Windows and into `errno` elsewhere.

On another note, I've noticed that in some parts of the codebase, if there's some error,
the caller checks for EAGAIN and EWOULDBLOCK, but in other parts only for one of them. TODO: fix.

------------------------------------------------------------------------------------------------------------------------------------------------------------------------

The files in Common/Net/NetBuffer.h/.cpp contain a queue-like buffer, which is essentially the `Buffer` from
Common/Buffer.h/.cpp, but with more functionality.

The `Buffer` from Common is pretty well documented in the header file, except for its `void_` field and `IsVoid` accessor.
These feel more like a dirty hack without documentation.
It literally isn't used by the buffer itself, just a tag that HTTPClient checks (added in https://github.com/hrydgard/ppsspp/commit/1459c16fb8fe86a8522b11e7f9921173b75e8c0f)
The way I see it, this is something like "don't bother with postprocessing (dechunking&ungzipping) the HTTP request data" and nothing else.

So. Let's assume that the reader is familiar with the `Buffer`'s API. The class NetBuffer adds a utility function to read from a socket and into the buffer!
"Read(sock_fd, size)". Actually, it's not entirely new stuff... It's just that originally there was no `NetBuffer` and its functionality was a part of `Buffer`.

TODO: the `if (retval < 0)` check is never gonna fire, is it? I feel like it's an old bug.
Another reason why I think it's bugged is because this function can NEVER return negative values,
however, `Client::ReadResponseHeaders` checks if the result is negative.

Next there is "ReadAllWithProgress(sock_fd, knownSize, request_progress_ptr)". It seems to be similar to the last function,
but it has some more args and more eh... optimizations?

`request_progress_ptr` is a pointer to a class `RequestProgress`, which acts as a cancellation token and also a view over the progress and the speed.
Or nullptr. "ReadAllWithProgress" works just fine with it equal to nullptr.

`knownSize` is not documented, but it seems to be the expected total number of bytes to read.
Its purpose is to help with setting up a buffer and to report the progress.

There is a comment `We're non-blocking and reading from an OS buffer, so try to read as much as we can at a time.`
Does this imply that this API is supposed to be used with non-blocking sockets?
I am also not sure where the magical numbers come from. Are these values cross-platform (across our supported platforms)?

The `RequestProgress` class is constructed from a pointer to bool that acts as a cancel flag (or nullptr to have no cancellation).
This pointer is stored inside the class. It's not atomic, not volatile... hmm... Whatever... It should be fine.


Before every call to `recv`, the code attempts to check for cancellation. If it's disabled, we just go to `recv`.
If not, we enter a loop that checks the flag and waits until the socket is ready with a small timeout to make sure we'll be able to check again, if it's not ready.
On every successful `recv`, the read data is pushed into the buffer with optional progress reporting.
This is done by calling the method "Update(read_bytes_count, total_bytes_count, is_done)".

If `total_bytes_count` is zero, assumes the completion advanced by 1%.
This class has an undocumented field `callback` (with signature `void(int64_t, int64_t, bool)`, which tells us nothing).

TODO: introduce a using? Maybe OnProgressUpdate? Because that's what this callback is for!
On every call to "Update", if there is a callback, it's invoked with the args `read_bytes_count, total_bytes_count, is_done`.


Lastly, there is "FlushSocket(sock_fd, timeout, cancelled_flag_ptr)"
So, "Read" reads (calls 'recv'). And "FlushSocket", logically (as if)... Calls 'send'. Because it's a function for sending data from the buffer into the socket.

The first arg is uintptr_t instead of int. OOOOOH! It's because the WinSock2 stuff uses uintptr_t for sockets...
But that raises another concern... Because "ReadAllWithProgress" accepts an int socket! And the only place where it's used passes in a uintptr_t, which gets truncated, lol.
Something's hella fishy here... TODO: investigate this inconsistency.

Now then, the logic of the function. Before each call to `send`, it waits until the socket's ready for writing (with cancellation and timeout checks).
It keeps sending data until the whole buffer is sent.

------------------------------------------------------------------------------------------------------------------------------------------------------------------------

The Common/Net/Resolver.h/.cpp files contain the DNS logic. No classes, only standalone functions.

The logs are messy: sometimes it's Log::IO and sometimes it's Log::sceNet. It should honestly be Log::IO..........
I mean, I get it, some stuff was moved from PSP networking code into PPSSPP networking code... TODO: fix this.

enum class DNSType {
	ANY = 0,
	IPV4 = 1,
	IPV6 = 2,
};

"Init()" and "Shutdown" are used to initialize the requires systems. Only on Win32, if we are to trust the comment.
However, I don't think we should. Because right now we have some all-platforms naett calls in there... TODO: rephrase it or delete.


"HostPortExists(host, port, timeout_ms)" - that one is well-documented... but for some reason only in the .cpp file! TODO: move to header?
By the way, this spans for 140 lines or so. Is this all really necessary?


"DNSResolve(host, service, out addrinfo_ptr_ptr, out error, dns_type = DNSType::ANY)"
This uses the `getaddrinfo` function to resolve the IP from host and service (port or "named port").


"DNSResolveFree(addrinfo_ptr)" - a function for "freeing" addrinfo.

"GetLocalIP4List(IPv4_addresses)" - added by Anr2me in commit https://github.com/hrydgard/ppsspp/commit/596fad3f424c05550a1fb664440c40409ba4eac6 called
`Provides quick IP fill on Adhoc Server IP address setting to makes player's life easier :)`.
This function gets the local computer's host name and then turns it into a list of IP addresses, which are *appended* to the input vector of strings.

It used to be called `GetIPList`... The logs still refer to it as `GetIPList`!
Like `ERROR_LOG(Log::IO, "GetIPList failed to create socket (result = %i, errno = %i)", sd, socket_errno);`

This function uses `push_back` and `emplace_back` inconsistently.
Intended error handling for `::gethostname` was never implemented.


"inet_pton(addr_family, src, dst)" - parses the source IP string into dest buffer (4 or 16 bytes). The addr_family is for IPv4/IPv6 switch.

I don't know why we have a custom `inet_pton` in namespace `net`...
For example, the `inet_pton` called from `sceNetInetInetPton` is the actual one and not this one.


// Does a DNS lookup without involving the OS, so you can hit any DNS server.
"DirectDNSLookupIPV4(dns_server_ip_str_ptr, domain_str_ptr, ipv4_addr_ptr)" - does almost what the comment says.
This constructs a DNS query, sends it, recvs from the server and parses the response (it fetches the first IP from the response).
However, it also uses a cache "{dns_server_ip}:{domain_str_ptr}" -> IPv4, which is tried first. If not found, proceeds with the DNS query.
There is no way to bypass the cache or avoid saving the results.

TODO: move the actual DNS code into some "contact_dns_server" function or something.

By the way, the header and the cpp file have different arg names. I wonder if there are any other cases like this. TODO: fix.


------------------------------------------------------------------------------------------------------------------------------------------------------------------------

For issueing HTTP requests (like file downloads or post commands), there is a class RequestManager. And also abstract class Request.
There is a global instance of RequestManager (called "g_DownloadManager"), oddly placed in Config.h/.cpp files.

Config.h:
// TODO: Find a better place for this.
extern http::RequestManager g_DownloadManager;

Config.cpp:
// TODO: Find a better place for this.
http::RequestManager g_DownloadManager;


And there's `HttpImageFileView` in UI/Store.cpp, which stores a pointer to RequestManager as a field.
But we pass `&g_DownloadManager` to it, so it's the same RequestManager.
Common/Net/HTTPRequest.h contains a comment that indirectly implies that this *class* is supposed to be a singleton for the whole app.
That is, next to the Request's `SetCallback` method:
```
// NOTE: Completion callbacks (which these are) are deferred until RunCallback is called. This is so that
// the call will end up on the thread that calls g_DownloadManager.Update().
```



enum class RequestFlags {
	Default = 0,
	ProgressBar = 1,
	ProgressBarDelayed = 2,
	Cached24H = 4,
	KeepInMemory = 8,
};
ENUM_CLASS_BITOPS(RequestFlags);


RequestManager's API:
Configuration setters: "SetUserAgent(string_view)", "SetCacheDir(path)".

The cache dir is a real filesystem directory that is used to cache file downloads.
The manager can be instructed to check the cache dir first before making a network request.


Starting a download:
"StartDownload(url, outfile_path, request_flags, accept_mime, name, callback)"

The idea is that the download will complete at some point in the future (or error out) and then the manager will call `callback(request)`.
The semantics remind me of futures.
The callbacks can start new downloads while running!
This function returns a shared_ptr to Request (the abstract class mentioned above).



If the file is cached (and not older than 24 hours), it returns a "fake" CachedRequest.
Otherwise we create a new request, configure it, call `Start` and return it.

No matter what, the request is pushed into an internal vector `newDownloads_`.


Issuing a POST request:
"AsyncPostWithCallback(url, post_data, post_mime, request_flags, callback, name)".
This one creates a request, configures it, calls `Start`, pushes a copy into `newDownloads_` and returns it.


"Update()"
Without this function the callbacks woulnd't be run at all.
It flushes the `newDownloads_` into `downloads_` and then iterates over `downloads_` to check if any of them are ready.
If yes, then it runs the callback, calls `dl->Join()`, erases the item and starts scanning from the start.

TODO: investigate this place. The flushing clearly can be made with std::move from <algorithm>.
The remove-erase idiom for `downloads_` might not be applicable because of the special semantics, but then a comment wouldn't hurt.

This function is called from `StoreScreen::update` and `NativeFrame`.


"CancelAll()"
Simply calls `Cancel` for all items in `downloads_`, then `Join`. Lastly, it clears the vector.

Apparently StoreScreen's dtor calls g_DownloadManager.CancelAll()... As if there can't be anything else?
Same with FrameDumpTestScreen's dtor.
TODO: check

The only other place is in RequestManager's dtor, which is expected.


"UrlToCachePath(url)"
This one doesn't actually check if the file is available, it simply returns the path where the cached file would be.

"ReadFileFromCache(url, out_data)" is a utility function that implements a common pattern of fetching the cached file and reading its contents to std::string.
TODO: use it for jsonUrl in infra downloads (`sceNet.cpp#L439`)


TODO: STR_VIEW for logs + fix the logging group from Log::sceNet to something else (in the line `"Returning cached file for %.*s: %s"`)

Documentation on Request and its children ahead:

enum class RequestMethod {
	GET,
	POST,
};

Nothing else is supported by the manager, however, the HTTP client does support that.


The RequestManager returns a shared_ptr to Request from its `StartDownload` and `AsyncPostWithCallback`.
The request object is created in the "factory" static function.
"CreateRequest(http_method, url, post_data, post_mime, outfile_path, request_flags, resolve_func, name)".
So it has the necessary args for the download and for the POST request... Mixed together in a pile.

However, this function exists solely for handling HTTPS. If the URL is https, PPSSPP tries to create a concrete Request class "HTTPSRequest".
If a macro HTTPS_NOT_AVAILABLE is set, an empty shared_ptr to Request is returned. If the URL is http, it creates a concrete Request class "HTTPRequest".

The Request class is actually quite a big object.
It stores the request properties (for example, the HTTP method, the URL, the outfile_path) and some utility objects,
for instance, an instance of RequestProgress, a callback and an instance of a `Buffer` from Common to store the result (if not downloading from a file).

The constructor sets a callback for the request progress (the callback for reporting the progress updates).
TODO: Reaaaaaaally reconsider the naming. Just `callback` doesn't tell shit.

This callback constructs a string for a message and then, if there's a flag `RequestFlags::ProgressBar`, update the OSD message or remove it.
TODO: Honestly, it shouldn't prepare the message and THEN check if it's required... That's just useless work.

Other than the setters and simple getters, there is a function "RunCallback()" for invoking the stored callback and 4 pure virtual functions:
"Start()", "Join()", "Done()" and "Failed()". These are implemented in the children. And not documented. TODO?

From the children's implementations I've gathered that "Start" is seemingly expected to start the download asynchronously (or rather by spawning a thread),
"Join" is expected to join the thread. "Done" is for checking if the networking activity has completed. And sometimes for polling.
"Failed"... eh... The meaning differs across implementations right now. One thing is clear - being cancelled implies "Failed()".

More on the children later.

------------------------------------------------------------------------------------------------------------------------------------------------------------------------

The outgoing HTTP connections are implemented in HTTPClient.h/.cpp.

class Connection;
I don't think this is currently used on its own... It's inherited from by literally one class (http::Client) and that's it.

It has a field `host_` which a nearby comment calls "remote host". TODO: maybe rename?

It's using a properly named functor `ResolveFunc` equal to `std::function<std::string(const std::string &)>`.
The Connection's API:
"Connection(resolve_func)" - the constructor from a resolver. The kind of that doesn't move it into the field and copies instead... Sigh...

"Resolve(host_ptr, port, dns_type)" - resolves the host into the field `resolved_`. The comment says `// Inits the sockaddr_in.`
Calls the resolver if present. The only custom resolver that I've found is `ProcessHostnameWithInfraDNS` from sceNet.cpp.
Even if the resolver is able to find the IP address, the function calls net::DNSResolve on the result no matter what.

It seems that this function makes way too many copies of std::string (and also constructions from const char*, which triggers multiple strlen calls).
TODO: replace some uses of `host` to `host_`. And also this: if there is no custom resolver, `processedHostname` is not needed.
And if there is a resolver... it's also not needed, because it'll be overwritten.


"Connect(maxTries, timeout_per_try, cancelled_ptr)" - attempts to create a TCP non-blocking connection (initializes `sock_`).
Each attempt goes as follows:
It quickly iterates over the `resolved_` addresses (it's a linked list with `ai_next` ptr), creates a socket for each,
makes it non-blocking, calls `connect` and pushes the socket into a vector.
Then it enters a loop that uses `select` with a timeout (up to 0.5 seconds) to see if anything managed to connect. It also checks for cancellation.
If nothing connected, close all sockets, check for cancellation and go to the next attempt after 1 second of sleeping.
If something connected, pick the first one and close the others.


Oddly uses both Log::IO and Log::HTTP. This class is technically not tied to HTTP, so I'd rather change that to IO.
TODO: rename timeout to timeoutPerTry or timeoutPerAttempt.


"Disconnect()" - closes the socket `sock_`.

The dtor simply calls `Disconnect` and `DNSResolveFree` on the `resolved_` if not nullptr.


There are also 2 accessors.

"sock()" returns `sock_`. The comment says `// Only to be used for bring-up and debugging.`.
I don't know what "bring-up" means, but this method is used during the data sending.


"GetLocalIpAsString()" returns a string representation of the local ip (internally `fd_util::GetLocalIP` on `sock_`, which uses `getsockname`).


class RequestParams;
That's a simple wrapper over the resource string and accept_mime.


class Client;
This is the HTTP client: it inherits from `Connection` and stores the user agent, the HTTP version and a field `dataTimeout_`.
Honestly, it's a "header recv timeout".

Client's API:

"Client(resolver)" - the constructor from a resolver. It's passed by value to the base class (screaming).

The aforementioned fields have dedicated setter methods.

The most low-level API consists of "SendRequestWithData(method, request_params, data, additional_headers_ptr, request_progress_ptr)",
"ReadResponseHeaders(read_buffer, out_response_headers, request_progress_ptr, out_first_line)" and
"ReadResponseEntity(read_buffer, in_response_headers, output_buffer, request_progress_ptr)".

They do what they say... One note though... For some reason they all *implicitly* assume that the request_progress_ptr is not nullptr!
Contrary to the preconditions of net `Buffer`'s `ReadAllWithProgress`!

`SendRequestWithData` constructs the HTTP 1.1 request (header + the provided data), printfs it into the buffer and flushes the buffer into the `Connection`'s `sock_`.
This method doesn't verify the method string's validity. No keepalive, just "close". The default user-agent is "PPSSPP".
The host is picked from the field `host_`. The additional headers, if any, are inserted at the end of the header. The function makes sure to check for cancellation.

`ReadResponseHeaders` waits until the socket is ready with cancellation checks (or until the `dataTimeout_` expires),
then tries to read at most 4096 bytes into the buffer...

The comment says `Let's hope all the headers are available in a single packet...`. I don't get it, isn't the "expected" MTU ~1500 bytes?
But yes, if the headers take more than 4096 bytes, PPSSPP simply assumes that the rest is a part of the data.
This won't end well for whomever is gonna parse the data... I don't think it's a probable scenario, not a priority.

So, the first line from is parsed to identify the response code and saved to `out_first_line` if it's not nullptr,
then the remaining lines are pushed into `out_response_headers`.

TODO: consider changing `responseHeaders.push_back(line);` to `responseHeaders.push_back(std::move(line));` and `*statusLine = line;` to `*statusLine = std::move(line);`


Lastly, `ReadResponseEntity` iterates over the headers looking for "Content-Length", "Content-Encoding" or "Transfer-Encoding" headers.
If it sees "Content-Length", then it attempts to parse the content length and remember that the response is not chunked.
If it sees "Content-Encoding", then it searches for "gzip" (sadly, case-sensitive, which is a TODO) and remembers that the response is gzipped.
If it sees "Transfer-Encoding", then it searches for "chunked" (sadly, case-sensitive, which is a TODO) and remembers that the response is chunked.
If the output buffer doesn't have the "void" tag, performs the necessary dechunking&ungzipping

For some reason this method strengthens the `request_progress_ptr` requirements even further!
There's a `_dbg_assert` that the cancel token `progress->cancelled` is not nullptr! Reminder: this is stronger than `ReadAllWithProgress`'s requirements!

Both `ReadResponseHeaders` and `ReadResponseEntity` sometimes access std::string::npos through the object. TODO: `line.npos` -> `std::string::npos`.


Now onto the high-level API:

"SendRequest(method, request_params, additional_headers_ptr, request_progress_ptr)" same as `SendRequestWithData`, but with no data.

"POST(request_params, data, mime, output_buffer, request_progress_ptr)" - makes a POST request.
First it prepares the additional headers ("Content-Length" and "Content-Type" if mime is not empty), then makes the actual request.
Lastly, it processes the response (the entity is written to the `output_buffer` and the headers are then discarded).

"POST(request_params, data, output_buffer, request_progress_ptr)" - makes a POST request, but with no `mime` arg.

"GET(request_params, output_buffer, out_response_headers, request_progress_ptr)" - makes a GET request.
First it prepares the additional headers ("Accept-Encoding: gzip"), then makes the actual request.
The `out_response_headers` are populated while it processes the response (and `output_buffer` is where the entity goes).

"GET(request_params, output_buffer, request_progress_ptr)" - makes a GET request, but with no `out_response_headers` arg.



Because these methods internally call `ReadResponseEntity` or `SendRequestWithData`, the `request_progress_ptr` arg has strong *implicit* preconditions.

Do we properly support stacked (layered) content encodings? Like "deflate, gzip"? From what I can tell, we don't.
And we also don't force-configure the "Accept-Encoding" to disable this. Except in the very high-level `Client::GET`.
TODO: I think this is worth mentioning.


TODO: use STR_VIEW in `Client::POST`.


Lastly, there are 2 standalone functions in namespace `http`.
"GetHeaderValue(in_response_headers, header, out_value)" - a generic routine for fetching HTTP header values... except for "Set-Cookie".
`// Ignores line folding (deprecated), but respects field combining.`
The comment is in the .cpp file instead of the header. TODO: move?


"RemoveHttpsIfNeeded(url)" - replaces the "https" part with "http" if `System_GetPropertyBool(SYSPROP_SUPPORTS_HTTPS)` fails.
Interestingly, this doesn't check the macro HTTPS_NOT_AVAILABLE and makes a system request instead. I don't know what's better.

------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Now then, time for `Request`'s children.

class HTTPRequest: Request;

"HTTPRequest(method, url, post_data, post_mime, outfile_path, request_flags, resolve_func, name)" - the constructor.
For some reason doesn't use the member initializer list for `outfile_`.

The abstract methods "Start()", "Join()", "Done()" and "Failed()" are implemented here.

`Done` and `Failed` simply return `completed_` and `failed_`, respectively.

The `Start` method spawns an OS thread running the `Do` method and `Join` joins it if it's joinable (or `_dbg_assert(false)`!)

The `Do` method sets the wrong thread name "HTTPDownload::Do" (the class was called differently before) instead of "HTTPRequest::Do".
It calls `Perform` to actually fetch the URL. If the code is -1 (obviously not a real HTTP code, just an error value), `SetFailed(code)` and return.
If the code is one of the redirection codes, we try to grab the new URL. If we can't, we bail out without setting `failed_` to true.
The redirection code is saved to `resultCode_`. If we can, we compare the URL to the original URL and the current URL. If a loop is detected, we bail out.
Again, `failed_` remains false, the redirection code is saved, no logs are emitted. If everything's fine, we do another download attempt with the new URL.

This continues until the response code is not a redirect (or until it fails). If it didn't fail, we compare the code to 200.
If it's 200, we flush the download to a file if `outfile_` is not empty (and if it is, we keep the data in the `buffer_`).
If it's not, we emit an error log. In both cases, the response code is saved.

Important note: checking for the original URL and the current URL is not enough to detect cycles. It's just a simple implementation that handles most of the cases.
`URL1 -> URL2 -> URL3 -> URL2` would hang this code and someone would have to use the cancellation mechanism to terminate it.

Another note: the actual redirect codes that PPSSPP reacts to are `301 moved permanently`, `302 found`, `303 see other`,
`307 temporary redirect` and `308 permanent redirect`. PPSSPP seemingly breaks some HTTP rules,
because `303 see other` demands that the redirected resource is accessed with GET even if the original request method was POST.

One more thing... The redirection url is constructed *relative to the original URL*! I have no clue if it's bad or not. TODO: investigate.

Also an interesting observation... Getting 404 or something like that doesn't make the code set `failed_` to true.

And another note: the private method `SetFailed(http_code)` for some reason doesn't use its only argument!

The dtor removes the OSD progress bar (which is also done by the `RequestProgress`'s callback, but I guess it doesn't run in all execution paths where errors occur)
and makes a `_dbg_assert_msg_` check if the thread is joinable.


The private method `Perform` creates an instance of `http::Client`, configures it, calls `Resolve`, checks for cancellation,
connects with 2 attempts and 20 ms timeout, checks for cancellation, then calls either `client.GET` or `client.POST` (depending on the method).



class CachedRequest: Request;
"CachedRequest(method, url, name, cancelled_ptr, request_flags, response_data)" - inits the abstract base and pushes the response_data into the buffer.
Its `Start()` and `Join()` are no-ops. It's always `Done()` and never `Failed()`.



There's also HTTPNaettRequest.h/.cpp. If the macro HTTPS_NOT_AVAILABLE is defined, they don't store anything. Otherwise...


class HTTPSRequest: Request;

Quick note: this class uses Log::IO. I think it should use Log::HTTP in all places.
Now onto the methods!

"HTTPSRequest(method, url, post_data, post_mime, outfile, request_flags, name)" - the constructor.
For some reason doesn't use the member initializer list for `outfile_`. Just like `HTTPRequest`. Ctrl+C, Ctrl+V?

Also for some reason stores a field `method_` that duplicates the abstract base's `method_`. I have no idea why.
TODO: It probably should just use the parent's field.

"Start()" - creates a vector of naett options (actually pointers, but whatever),
calls `naettRequestWithOptions` and `naettMake` (the return values are saved), then updates the progress as 0%.
I assume that naett spawns a thread on its own.

The naett timeout is 30 seconds.

TODO: use `RequestMethodToString` instead of a ternary operator `naettMethod(method_ == RequestMethod::GET ? "GET" : "POST")` (not extensible).

"Join()" immediately returns if there is no pending operation. Otherwise it tears the naett stuff down if `completed_`.
TODO: the else branch emits the log `"HTTPSDownload::Join not implemented"`. First of all, this is... AGAIN! The wrong name!
Second of all, it shouldn't log that. I mean, it *is* implemented, isn't it?

"Failed()" simply returns `failed_`.

"Done()" though... It's used for checking if the naett is ready. If `completed_` is already set, hooray, return true. If not, we ask naett if it's ready.
If not, we ask the progress report, update our progress and return false. If it is ready, then we do something really odd.
If we're cancelled, we set `resultCode_` to -1000. There is a comment:
`// -1000 is a code specified by us to represent cancellation, that is unlikely to ever collide with naett error codes.`
If we're not cancelled, we set `resultCode_` to naett's status code... After this we extract the body into the `buffer_`.

Note: the response body is extracted even when the response isn't 200 because of rcheevos.
See https://github.com/hrydgard/ppsspp/commit/78de59a47940cf1218b781f68da9a1216049aa9f

If the code is negative (some error), we make a switch... that never handles the -1000 case!!!
Therefore -1000 goes to the default branch which logs "Unhandled naett error -1000". TODO: fix.
Anyway, `failed_` is set to true, then we update the progress to 100%.

If the code is 200, we flush the download to a file if `outfile_` is not empty (and if it is, we keep the data in the `buffer_`).

Otherwise we WARN_LOG the "Naett request failed" with the code, set `failed_` to true and update the progress as completed.

In all 3 scenarios we set `completed_` to true and return true.



Note: the non-200 responses are treated as failures here, contrary to the HTTPRequest class!!!
By the way, I don't know if naett can handle redirects. I literally don't know.

Note: the exact same scenario (404 and such) produces error logs and not warning logs for HTTPRequest. Another inconsistency!

------------------------------------------------------------------------------------------------------------------------------------------------------------------------