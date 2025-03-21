jest.setTimeout(120000);

import { CreatePostLike } from "lemmy-js-client";
import { alpha, setupLogins, createPost, createCommunity, registerUser, alphaUrl } from "./shared";

beforeAll(async () => {
  await setupLogins();
});

test("Rust allowed voters", async () => {
  let community = await createCommunity(alpha);
  let postRes = await createPost(
    alpha,
    community.community_view.community.id,
    "Voting",
  );
  expect(postRes.post_view.post.name).toBeDefined();

  // user has no posts and is not allowed to vote
  let new_user = await registerUser(alpha, alphaUrl, "new_user");
  let form: CreatePostLike = {post_id: postRes.post_view.post.id, score: -1};
  await expect(new_user.likePost(form)).rejects.toStrictEqual(Error("plugin_error"));

  // create a few posts
  for (var i= 0; i< 5; i++) { await createPost(
    new_user,
    community.community_view.community.id,
    "test post number " + i,
  );
  }

  // now user is allowed to vote
  let vote = await new_user.likePost(form);
  expect(vote.post_view.post_actions?.liked).toBeDefined();
});
