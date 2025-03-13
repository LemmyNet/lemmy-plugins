jest.setTimeout(120000);

import {
  alpha,
  setupLogins,
  createPost,
  unfollows,
  createCommunity,
} from "./shared";

beforeAll(async () => {
  await setupLogins();
});

afterAll(unfollows);

test("Plugin test", async () => {
  let community = await createCommunity(alpha);
  let postRes1 = await createPost(
    alpha,
    community.community_view.community.id,
    "https://example.com/",
    "Rust",
  );
  expect(postRes1.post_view.post.body).toBe("Go");

  await expect(
    createPost(
      alpha,
      community.community_view.community.id,
      "https://example.com/",
      "Java",
    ),
  ).rejects.toStrictEqual(Error("plugin_error"));
});
