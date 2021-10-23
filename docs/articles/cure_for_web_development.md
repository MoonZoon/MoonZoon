<img src="images/code_and_pills.jpg">

# Cure for Web Development

Imagine you have a small business. You sell flowers in your brick-and-mortar store. Everything goes well, but you would like to attract more customers and provide some extra services - e.g. deliver flowers to a given address.

So you need a website. Just some basic stuff - a home page, flower catalog, order form and maybe a simple chat and blog. 

You don't have enough money to hire a professional developer. You can't find a suitable website builder. Fortunately, you know something about computers so you decide to create the website by yourself.

The gates of hell have been opened.

---

You don't know where to start, but internet knows everything. After a while, you find some [ridiculously complicated diagrams](https://github.com/kamranahmedse/developer-roadmap) of the necessary knowledge.

Then the torment continues, but you find out you need to learn at least these things:
- HTML
- CSS
- Javascript
- JSON
- Git
- DNS
- HTTP
- REST
- WebSockets
- SQL
- Cookies / JWT

... to have an idea how to choose the right languages, libraries, frameworks and hostings.

You fall into the rabbit hole and then analysis-paralysis kicks in. Should you use:
- Javascript or Typescript?
- Bootstrap or Tailwind?
- React or Vue?
- Mongo or Postgres?
- Serverless or VPS?
- Monolith or Micro-services?
- Webpack or Parcel?
- Less or Sass?
- ...

Who knows? Nobody. Flame wars are very popular among web developers.

---

> It works, until you try to do the first major refactor - you'll feel like you are going through a minefield.

When you look at most frontend or backend frameworks and squint your eyes a bit, you'll see just a bunch of helpers for HTML / CSS / JSON manipulation and HTTP wrappers. 

But what if HTML and CSS are not suitable for web apps? They were designed for simple web pages a long time ago and basically cannot be improved due to backward compatibility. As the result, the number of CSS and HTML features is growing and almost no one is able to use them properly.

Then combine HTML with CSS and a language designed for writing short scripts - such as Javascript - and write a web app. It works, until you try to do the first major refactor - you'll feel like you are going through a minefield. Anything you touch may explode in the runtime. And no one is brave enough to even enter a CSS minefield and remove old CSS code.

So maybe there are so many frameworks because they do too clever HTML manipulations and want to be as composable and flexible and universal as possible but they fail to achieve the most important goal - to make the web development easier.

---

> I can imagine many developers pray for a small number of users

Another source of fun for an entire day is choosing database and eliminating single points of failure. There are basically two popular approaches:

1. Use one database (ideally a managed cluster) and one or more stateless apps in containers. Then hope it doesn't become a DevOps nightmare with random database deadlocks.

2. Use serverless functions and databases and hope you don't have cold starts, don't receive a surprising invoice and don't need real-time communication (e.g. WebSockets or Server-Sent Events).

I can imagine many developers pray for a small number of users so the infrastructure doesn't fall like a house of cards or burn all the money.

---

**There is a cure for this madness:**

- A statically typed language without footguns like `null`, `undefined` or inheritance. Fast and pragmatic. [Rust](https://www.rust-lang.org/).

- A frontend framework with a good API for page elements (HTML and CSS abstraction). It should motivate you to write accessible and SEO content. You shouldn't need to deal with low-level stuff (e.g. with communication protocols).

- A backend framework that manages data directly without a database. It can automatically join multiple server nodes to a cluster and use them transparently as one large server. With built-in authentication.

- A hosting / cloud with a reasonable and predictable pricing. With monitoring, logging, autoscaling and one-command deployment.

Let me introduce to you the Rust fullstack framework: [MoonZoon](https://moonzoon.rs).

<p align="center">
  <img src="images/MoonZoon.png" width="360" title="MoonZoon logo">
</p>

---

Images:
- [Code](https://unsplash.com/photos/MI9-PY5cyNs) by Markus Spiske
- [Pills](https://unsplash.com/photos/Nqj2XWHy4K0) by Kate Hliznitsova 
