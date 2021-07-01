# MoonZoon Cloud
---

MoonZoon Cloud - the serverless platform for your MoonZoon apps. Predictable pricing, simplicity and scalability are main goals.  

- The web administration will be written in MoonZoon.
- The CLI part will be implemented as extra `mzoon` commands.
- The default app URL will be `https://[your_app_name].mzoon.app`
- I hope it will be the main source of income for MoonZoon development.

If you want **early access** to the MoonZoon Cloud, [sponsor](https://github.com/sponsors/MartinKavik) me and write me a private message on [Discord](https://discord.gg/eGduTxK2Es). Don't hesitate to tell me about your plans and needed features.

_Current state_: Research. [Clever Cloud](https://www.clever-cloud.com/en/) looks like the best candidate for the MoonZoon Cloud infrastructure provider. The MZ chat example works without problems: [mz-chat-example.mzoon.app](https://mz-chat-example.mzoon.app/). Their support is excellent. Another possible provider could be [Google Cloud Run](https://cloud.google.com/run) because it supports scaling to zero and also server-sent events, but Clever Cloud has more predictable pricing and it's less complex because you don't need to manage Docker containers. However MZ Cloud can offer multiple providers in the future - it means you would be able to choose Clever or Google Cloud.

---

## FAQ
1. _"I want to deploy my MoonZoon app now!"_

   - There is a [buildpack](https://github.com/MoonZoon/heroku-buildpack-moonzoon) for Heroku and Heroku-like PaaS and the [demo](https://github.com/MoonZoon/demo).

   - It should be possible to modify buildpack's code and then setup, for instance, GitHub Actions as your own deploy pipeline to a hosting that supports Docker. Don't hesitate to write about your intentions on our [chat](https://discord.gg/eGduTxK2Es).

1. _"Why sponsoring instead of standard paid accounts / subscriptions / credits?"_
   - I want to focus on MoonZoon and Cloud development instead of writing or integrating systems for billing and related stuff.
   - Everybody will see you support open-source projects.
   - I'll switch to a standard in-app payments when it makes sense. 
