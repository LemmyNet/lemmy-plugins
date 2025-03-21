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

export function after_create_local_post() {
  const params = JSON.parse(Host.inputString());
  if (params["name"] != "Notification") {
    // Ignore posts with a different title, otherwise unrelated tests will throw
    // connection error as notification server is not running.
    return;
  }
  const url = Config.get("notify_url");
  const request: HttpRequest = {
    method: "POST",
    url: url!,
  };
  const response = Http.request(request, params["ap_id"]);
  console.log(response);
}
