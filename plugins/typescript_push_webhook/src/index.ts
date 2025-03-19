export interface Metadata {
  name: string;
  url: string;
  description: string;
}

export function metadata() {
  let metadata: Metadata = {
    name: "Test Plugin",
    url: "https://example.com",
    description: "Plugin to test Lemmy feature",
  };
  Host.outputString(JSON.stringify(metadata));
}

export function new_post() {
  const params = JSON.parse(Host.inputString());
  if (params["name"] != "Notification") {
    // Ignore posts with a different title, otherwise unrelated tests will throw
    // connection error as notification server is not running.
    return;
  }
  const request: HttpRequest = {
    method: "POST",
    url: "http://127.0.0.1:8927",
  };
  const response = Http.request(request, params["ap_id"]);
  console.log(response);
}
