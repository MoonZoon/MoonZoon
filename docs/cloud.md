# MoonZoon Cloud
---

MoonZoon Cloud - the serverless platform for your MoonZoon apps. Predictable pricing, simplicity and scalability are main goals.  

- The web administration will be written in MoonZoon.
- The CLI part will be implemented as extra `mzoon` commands.
- The default app URL will be `https://[your_app_name].mzoon.app`

If you want **early access** to the MoonZoon Cloud, [sponsor](https://github.com/sponsors/MartinKavik) me and write me a private message on [Discord](https://discord.gg/eGduTxK2Es). Don't hesitate to tell me about your plans and needed features.

_Current state_: Research. [Clever Cloud](https://www.clever-cloud.com/en/) looks like the best candidate for the MoonZoon Cloud infrastructure provider. The MZ chat example works without problems: [mz-chat-example.mzoon.app](https://mz-chat-example.mzoon.app/). Their support is excellent. Another possible provider could be [Google Cloud Run](https://cloud.google.com/run) because it supports scaling to zero and also server-sent events, but Clever Cloud has more predictable pricing and it's less complex because you don't need to manage Docker containers.

_Update:_ DigitalOcean's [App Platform](https://www.digitalocean.com/products/app-platform) is another interesting hosting alternative.

---

## FAQ
1. _"I want to deploy my MoonZoon app now!"_

   - [Dockerfile](https://docs.docker.com/engine/reference/builder/) will be included in the repo.

   - You can try to deploy to [Clever Cloud](https://www.clever-cloud.com/en/) by yourself. There is a chance I'll write a step-by-step guide later.

   - There is a [buildpack](https://github.com/MoonZoon/heroku-buildpack-moonzoon) for Heroku and Heroku-like PaaS and the [demo](https://github.com/MoonZoon/demo).

1. _"Why sponsoring instead of standard paid accounts / subscriptions / credits?"_
   - I want to focus on MoonZoon and Cloud development instead of writing or integrating systems for billing and related stuff.
   - Everybody will see you support open-source projects.
   - I'll switch to a standard in-app payments when it makes sense. 
