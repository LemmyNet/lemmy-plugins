jest.setTimeout(120000);

import {
  alpha,
  setupLogins,
  createPost,
  createCommunity,
  delay,
} from "./shared";
import { createServer, IncomingMessage, ServerResponse } from "http";

beforeAll(async () => {
  await setupLogins();
});

test("Typescript push webhook", async () => {
  var notificationReceived = false;
  let server = createServer(function (
    req: IncomingMessage,
    res: ServerResponse,
  ) {
    var body = "";
    req
      .on("data", chunk => {
        body += chunk;
      })
      .on("end", () => {
        console.log("New post with url: " + body);
        notificationReceived = true;
        res.writeHead(200);
        res.end();
      });
  });
  server.listen(8927);

  let community = await createCommunity(alpha);
  let postRes1 = await createPost(
    alpha,
    community.community_view.community.id,
    "Notification",
  );
  expect(postRes1.post_view.post.name).toBeDefined();

  while (!notificationReceived) {
    console.log(notificationReceived);
    await delay();
  }
  expect(notificationReceived).toBeTruthy();

  server.close();
});
