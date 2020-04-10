addEventListener("fetch", (event) => {
  event.respondWith(handleRequest(event.request));
});

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  const { handle } = wasm_bindgen;
  await wasm_bindgen(wasm);
  // trying to handle all with rust
  // returns `CODE BODY`: `200 ...`, what about the headers
  try {
    const resp = handle(request);
    // create init response from resp
    const params = {
      status: resp["status"] || 200,
      headers: new Headers(resp["headers"] || {}),
    };
    return new Response(resp["body"] || "", params);
  } catch (e) {
    return new Response(e, { status: 500 });
  }
}
