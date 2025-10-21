jest.setTimeout(120000);

import { NotificationView, PrivateMessageView } from "lemmy-js-client";
import {
  alpha,
  setupLogins,
  delay,
  isPluginActive,
  registerUser,
  alphaUrl,
  createPrivateMessage,
} from "./shared";
import { createServer, IncomingMessage, ServerResponse } from "http";

beforeAll(async () => {
  await setupLogins();
});

test("Typescript push webhook", async () => {
  await isPluginActive(alpha, "Push Webhook");
  var notif: string | null = null;
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
        notif = body;
        res.writeHead(200);
        res.end();
      });
  });
  server.listen(8927);

  // create private message which sends in notification to recipient
  let alphaProfile = await alpha.getMyUser();
  let otherUser = await registerUser(alpha, alphaUrl);
  let privateMessage = await createPrivateMessage(
    otherUser,
    alphaProfile.local_user_view.person.id,
  );
  expect(
    privateMessage.private_message_view.private_message.content,
  ).toBeDefined();

  // wait until notification webhook is received
  while (notif === null) {
    await delay();
  }

  // parse webhook result and compare
  let notif_parsed: NotificationView = JSON.parse(notif);
  expect(notif_parsed.notification.kind).toBe("private_message");
  expect(notif_parsed.data.type_).toBe("private_message");
  const pm = notif_parsed.data as PrivateMessageView;
  expect(pm.creator).toEqual(privateMessage.private_message_view.creator);
  expect(pm.recipient).toEqual(privateMessage.private_message_view.recipient);
  expect(pm.private_message).toEqual(
    privateMessage.private_message_view.private_message,
  );

  server.close();
});
