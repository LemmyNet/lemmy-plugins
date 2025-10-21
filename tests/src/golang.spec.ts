jest.setTimeout(120000);

import {
  alpha,
  setupLogins,
  createPost,
  createCommunity,
  isPluginActive,
} from "./shared";

beforeAll(async () => {
  await setupLogins();
});

test("Go replace words", async () => {
  await isPluginActive(alpha, "Replace Words");
  let community = await createCommunity(alpha);
  let postRes1 = await createPost(
    alpha,
    community.community_view.community.id,
    "Rust",
  );
  expect(postRes1.post_view.post.name).toBe("Go");

  await expect(
    createPost(alpha, community.community_view.community.id, "Java"),
  ).rejects.toThrow("We dont talk about Java");
});
