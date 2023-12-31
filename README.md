# 📸 Website: https://that.photo

## Photograph credit

The photograph featured in this website belongs to [Zoe Leonard](https://en.wikipedia.org/wiki/Zoe_Leonard). It is the inspiration for this work, and it is used without permission. Zoe Leonard's photography is remarkable, and I am grateful for her work questioning traditional views on gender and sexuality. I recommend anyone who reads this to investigate her art further.

## Technology

This website consists of two, almost seperate, projects. They were written about a month apart. These systems include:
* A backend system used to provide the polaroid photograph with video on the televisions
  * To see this portion of the project more clearly, you can click the red close button on the terminal to hide it.
* A nextJS app utilizing WASM compiled from Rust to compute the 2023 Advent of Code solutions

### Photograph

#### Intent

The intent of this art is to imagine what might have caused the photographer to stop and take this photo. The motivation is provided by "turning the televisions on" and finding a way to fill them with reasonable, possible video streams that are believable. Why is she taking that photograph? This is my imagining an answer to that question.

#### Mechanism

The photograph proceedes through the following steps on page load:
* A call to OpenAI's API is made to select a notable news event in a given year, randomly chosen between 1970 and now. The program picks the year, and GPT 4 picks the news event. This interval is chosen to provide a reasonably good chance of having news coverage of the event being available on YouTube.
* With a topic chosen, a search is made on Google's YouTube API to find semi-randomly chosen videos from the top results on that topic.
* With two videos chosen, each is downloaded to the server in 360p resolution.
* With mp4 files of the videos, each is run through a complex chain of `ffmpeg` filtergraphs to simulate being displayed on a CRT.
* As the processed video is produced, it is saved to disk as a TLS stream, allowing it to be used while the transcoding process continues.
* The web application polls for availability of enough of the TLS stream, and then begins to display it.
* When both video streams are playing topical videos through the above mechanism, the topic is displayed on the polaroid.
* While the TLS streams are being prepared, the televisions will alternate between brief flashes of videos depicting static and randomly, previously downloaded video, simulating a channel changing effect.
* The queing system used to pass events from one stage to the next is based on redis.
* Horizontal scalability is provided by standing the stack up in Kubernetes and using an archtecture based around redis queueing.
* An individual page load requires two cores of the processor to work for about two minutes, almost entirely dedicated to the `ffmpeg` processing. The cost of running this website at any level of scale would be brutal in the cloud. The current implementation is running on a real, metal `i7-10700` and can support two to three simultatinious visitors with transcoded video faster than the playback speed.
  * When you use this website, a computer in my living room spins up its fans. 🖥️💨

### Terminal

#### Intent

Pretty simply, the intent is to learn how to program in Rust. Beyond that, it's to execute Advent of Code solutions at near native, binary speed in the context of a client's browser on the client's hardware.

#### Mechanism

* I found this book, [Rust 🦀 and WebAssembly 🕸](https://rustwasm.github.io/docs/book/introduction.html), which inspired me to learn about compiling rust into web assembly.
* Inside the `rust` directory, there is a rust project which they are used to compile web-assembly (WASM).
  * These binary objects are able to be executed by the browser at near native binary speed, comperable to a locally compiled software's execution rate.
  * These binary WASM objects are symlinked into a nextjs app where it is run via the `<Rust />` component.
* The rust WASM code is run in a web-worker to avoid blocking the main JS event loop. They communicate with the main components through web-worker-like pub/sub methods. This architecture frees us up from needing to write async code, and we can plow ahead functionally without impact on the UI/UX, keeping the polaroid mechanisms working as intended. This is adventagous when you're trying to just crank out an AOC solution.
* The rust routines are provided a mechanism to print "status updates", similar conceptually to `console.log()` statements, as it proceedes through the solution to the program.
* These updates are collected and displayed in the `<Terminal />` component borrowed from [last year's AOC website](https://snowday.gg).
* The result is that the client's web browser provides the compute for the solution, and does it through execution of "binary" files compiled from rust at compile time, not execution time.
* Recompilation of the rust source into WASM modules interacts with the NextJS development cycle as you'd expect. When you recompile, the module is re-run, outputting its status updates to the terminal.

### Deployment

This is wreckless, "ride or die" deployment. Production and development environments are the same thing. If you see something break, just wait a second, I'm probably fixing it.
