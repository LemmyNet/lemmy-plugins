import type { Register } from "lemmy-js-client";

export interface Metadata {
  name: string;
  url: string;
  description: string;
}

export function metadata() {
  let metadata: Metadata = {
    name: "Block Spam Registrations",
    url: "https://github.com/LemmyNet/lemmy-plugins/",
    description: "Reject registrations from spam email domains",
  };
  Host.outputString(JSON.stringify(metadata));
}

export function local_user_before_register() {
  const data: Register = JSON.parse(Host.inputString());
  const email = data.email?.toString();
  const config = Config.get("blocked_domains") || "";
  let email_domain = email?.toLowerCase();
  for (let domain of config.split(",")) {
    domain = domain.trim().toLowerCase();
    if (domain.length > 0 && email_domain?.endsWith(`@${domain}`)) {
      throw new Error("email domain is not allowed to register");
    }
  }
  Host.outputString(JSON.stringify(data));
}
