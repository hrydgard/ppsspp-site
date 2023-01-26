export async function jsonFetch(apiName, requestBody) {
  return fetch('/api/' + apiName, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: requestBody ? JSON.stringify(requestBody) : null,
  }).then(data => {
    if (data.status == 200) {
      return data.json();
    } else {
      return null;
    }
  });
}

// This one doesn't expect a JSON response, just a status.
export async function jsonPost(apiName, requestBody) {
  return fetch('/api/' + apiName, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: requestBody ? JSON.stringify(requestBody) : null,
  }).then(data => {
    return data.status == 200;
  });
}