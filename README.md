# GPT4 Site Generator

Little experiment with the openai APIs to generate encyclopedia-style pages on the fly.

Works with GPT3.5-turbo as well, if you don't want to bankrupt yourself.

# Instructions

gpt4-site-generator requires an environment variable `OPENAPI_KEY` to be present to run. If it is not
set, the service will immediately shut down with an explanation.

To build/run:

`OPENAPI_KEY=YOUR_KEY_HERE cargo run`

Access the service at `localhost:3000/:category/:topic`. The page will be generated on the fly, and 
cached for repeated visits to the same page (in-memory cache - restarting the service will drop the 
cache!)

# Warning

This service racks up a bill quickly. It can cost as much as a few cents per request. Deploying this
to a publically available URL is not advisable. Furthermore, generating each site is very slow.

If speed and cost is a problem, it also supports GPT3.5-turbo, which, while still slow, is much less
slow than GPT4, and signifcantly cheaper as well. However, the quality is not as good.
