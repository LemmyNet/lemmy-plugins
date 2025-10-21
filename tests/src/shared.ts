import {
  CreatePrivateMessage,
  LemmyHttp,
  PersonId,
  PrivateMessageResponse,
  Register,
} from "lemmy-js-client";
import { CreatePost } from "lemmy-js-client/dist/types/CreatePost";
import { EditSite } from "lemmy-js-client/dist/types/EditSite";
import { Login } from "lemmy-js-client/dist/types/Login";
import { PostResponse } from "lemmy-js-client/dist/types/PostResponse";
import { CommunityResponse } from "lemmy-js-client/dist/types/CommunityResponse";
import { FollowCommunity } from "lemmy-js-client/dist/types/FollowCommunity";
import { CreateCommunity } from "lemmy-js-client/dist/types/CreateCommunity";
import { GetCommunity } from "lemmy-js-client/dist/types/GetCommunity";
import { LoginResponse } from "lemmy-js-client/dist/types/LoginResponse";

export const fetchFunction = fetch;

export const alphaUrl = "http://127.0.0.1:8541";
export const betaUrl = "http://127.0.0.1:8551";

export const alpha = new LemmyHttp(alphaUrl, { fetchFunction });
export const alphaImage = new LemmyHttp(alphaUrl);
export const beta = new LemmyHttp(betaUrl, { fetchFunction });

const password = "lemmylemmy";

export async function setupLogins() {
  let formAlpha: Login = {
    username_or_email: "lemmy_alpha",
    password,
  };
  let resAlpha = alpha.login(formAlpha);

  let formBeta: Login = {
    username_or_email: "lemmy_beta",
    password,
  };
  let resBeta = beta.login(formBeta);

  let res = await Promise.all([resAlpha, resBeta]);
  alpha.setHeaders({ Authorization: `Bearer ${res[0].jwt ?? ""}` });
  alphaImage.setHeaders({ Authorization: `Bearer ${res[0].jwt ?? ""}` });
  beta.setHeaders({ Authorization: `Bearer ${res[1].jwt ?? ""}` });

  // Registration applications are now enabled by default, need to disable them
  let editSiteForm: EditSite = {
    registration_mode: "open",
    rate_limit_message_max_requests: 999,
    rate_limit_post_max_requests: 999,
    rate_limit_register_max_requests: 999,
    rate_limit_image_max_requests: 999,
    rate_limit_comment_max_requests: 999,
    rate_limit_search_max_requests: 999,
  };
  await alpha.editSite(editSiteForm);
  await beta.editSite(editSiteForm);

  // Create the main alpha/beta communities
  // Ignore thrown errors of duplicates
  try {
    await createCommunity(alpha, "main");
    await createCommunity(beta, "main");
    // wait for > INSTANCES_RECHECK_DELAY to ensure federation is initialized
    // otherwise the first few federated events may be missed
    // (because last_successful_id is set to current id when federation to an instance is first started)
    // only needed the first time so do in this try
    await delay(10_000);
  } catch {
    //console.log("Communities already exist");
  }
}

export async function createPost(
  api: LemmyHttp,
  community_id: number,
  // use example.com for consistent title and embed description
  name: string = randomString(5),
  alt_text = randomString(10),
  custom_thumbnail: string | undefined = undefined,
): Promise<PostResponse> {
  let form: CreatePost = {
    name,
    alt_text,
    community_id,
    custom_thumbnail,
  };
  return api.createPost(form);
}

export async function createPrivateMessage(
  api: LemmyHttp,
  recipient_id: PersonId,
): Promise<PrivateMessageResponse> {
  let form: CreatePrivateMessage = {
    recipient_id,
    content: randomString(5),
  };
  return api.createPrivateMessage(form);
}

export async function followCommunity(
  api: LemmyHttp,
  follow: boolean,
  community_id: number,
): Promise<CommunityResponse> {
  let form: FollowCommunity = {
    community_id,
    follow,
  };
  const res = await api.followCommunity(form);
  await waitUntil(
    () => getCommunity(api, res.community_view.community.id),
    g => {
      let followState = g.community_view.community_actions?.follow_state;
      return follow ? followState === "accepted" : followState === undefined;
    },
  );
  // wait FOLLOW_ADDITIONS_RECHECK_DELAY (there's no API to wait for this currently)
  await delay(2000);
  return res;
}

export async function createCommunity(
  api: LemmyHttp,
  name_: string = randomString(10),
): Promise<CommunityResponse> {
  let description = "a sample description";
  let form: CreateCommunity = {
    name: name_,
    title: name_,
    description,
  };
  return api.createCommunity(form);
}

export async function getCommunity(
  api: LemmyHttp,
  id: number,
): Promise<CommunityResponse> {
  let form: GetCommunity = {
    id,
  };
  return api.getCommunity(form);
}

export async function loginUser(
  api: LemmyHttp,
  username: string,
): Promise<LoginResponse> {
  let form: Login = {
    username_or_email: username,
    password: password,
  };
  return api.login(form);
}

export async function registerUser(
  api: LemmyHttp,
  url: string,
  username: string = randomString(5),
): Promise<LemmyHttp> {
  let form: Register = {
    username,
    password,
    password_verify: password,
    show_nsfw: true,
  };
  let login_response = await api.register(form);

  expect(login_response.jwt).toBeDefined();
  let lemmy_http = new LemmyHttp(url, {
    headers: { Authorization: `Bearer ${login_response.jwt ?? ""}` },
  });
  return lemmy_http;
}

export function delay(millis = 500) {
  return new Promise(resolve => setTimeout(resolve, millis));
}

export function randomString(length: number): string {
  var result = "";
  var characters =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";
  var charactersLength = characters.length;
  for (var i = 0; i < length; i++) {
    result += characters.charAt(Math.floor(Math.random() * charactersLength));
  }
  return result;
}

export async function waitUntil<T>(
  fetcher: () => Promise<T>,
  checker: (t: T) => boolean,
  retries = 10,
  delaySeconds = [0.2, 0.5, 1, 2, 3],
) {
  let retry = 0;
  let result;
  while (retry++ < retries) {
    try {
      result = await fetcher();
      if (checker(result)) return result;
    } catch (error) {
      console.error(error);
    }
    await delay(
      delaySeconds[Math.min(retry - 1, delaySeconds.length - 1)] * 1000,
    );
  }
  console.error("result", result);
  throw Error(
    `Failed "${fetcher}": "${checker}" did not return true after ${retries} retries (delayed ${delaySeconds}s each)`,
  );
}

export async function isPluginActive(api: LemmyHttp, name: string) {
  let site = await api.getSite();
  if (!site.active_plugins.find(a => a.name == name)) {
    throw Error("Plugin {name} is not active");
  }
}
