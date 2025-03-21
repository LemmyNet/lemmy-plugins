jest.setTimeout(120000);

import { CreatePostLike } from "lemmy-js-client";
import { alpha, setupLogins, createPost, createCommunity } from "./shared";

beforeAll(async () => {
  await setupLogins();
});

test("Rust allowed voters", async () => {
  let community = await createCommunity(alpha);
  let postRes1 = await createPost(
    alpha,
    community.community_view.community.id,
    "Voting",
  );
  expect(postRes1.post_view.post.name).toBeDefined();
  let my_user = await alpha.getMyUser();
  console.log(my_user.local_user_view.person.id);

  let form: CreatePostLike = {post_id: postRes1.post_view.post.id, score: -1};
  let like = await alpha.likePost(form);
  console.log(like.post_view.post_actions?.liked);
  expect(like.post_view.post_actions?.liked).toBeDefined();
});
