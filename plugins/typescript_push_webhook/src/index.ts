export interface Metadata {
  name: string;
  url: string;
  description: string;
}

export function metadata() {
  let metadata: Metadata = {
    name: "Push Webhook",
    url: "https://example.com",
    description: "Plugin to test Lemmy feature",
  };
  Host.outputString(JSON.stringify(metadata));
}

export function notification_after_create() {
  const url = Config.get("notify_url");
  const request: HttpRequest = {
    method: "POST",
    url: url!,
  };
  const params = Host.inputString();
  const response = Http.request(request, params);
  console.log(response);
}
