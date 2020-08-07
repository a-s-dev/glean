# Nimbus Experimenter

## What is this
As a part of preperation for the Rust Nimbus SDK, this was built as a small prototype to uncover any issues or any unexpected blockers. 
This README will serve as a document explaining what we learned, how you can test this, what is missing and what we would might need to think hard about.


### Code organization
So far, the `nimbus-experiments` crate lives inside the `glean-core` crate. This is not where we expect the actual SDK to live, but it was convienient for prototyping other aspects of the project. There is current dicussion on where the actual crate will live, but for now this seemed like a quick spot for it us to try it out.

The crate is broken down to a few modules, each has a small description on top describing what it is for, and a few `TODO`s explaining what still needs to be done. The most notable one is the `ffi` which defines simple `C` apis, that later get re-exported in `glean-core/ffi`

The Android side of this crate is implemented in `glean-core/android` it was quicker to leverage the already setup infra for glean for the prototype, although, the full implementation will want to expose it's own bindings. 

## How can I try it out?
This PR adds all the `mavenLocal` needed to publish glean locally, which you can then use in `Android Components` to run a quick test to see the flow working.
Follow the following steps:
1. Make sure you are on the `test-experiments-do-not-merge` branch of [this](https://github.com/a-s-dev/glean) fork of glean:
   
    run `git clone git@github.com:a-s-dev/glean.git && cd glean`

    Then `git checkout test-experiments-do-not-merge`
2. Run `./gradlew publishToMavenLocal` in the root directory of **glean**
3. Checkout [this](https://github.com/a-s-dev/android-components) fork of `Android Components`:
   
    run `git clone git@github.com:a-s-dev/android-components.git`

    Then `git checkout test-experiments-do-not-merge`
4. In the `samples-glean` application in Android Components, open [`GleanApplication.kt`](https://github.com/a-s-dev/android-components/blob/test-experiments-do-not-merge/samples/glean/src/main/java/org/mozilla/samples/glean/GleanApplication.kt) and set a break point [here](https://github.com/a-s-dev/android-components/blob/test-experiments-do-not-merge/samples/glean/src/main/java/org/mozilla/samples/glean/GleanApplication.kt#L49)

## So what did you really learn?
1. We'll need to think a bit about networking. The prototype uses `viaduct` that allows the Kotlin consumer to set the `HttpClient` This works okay, except we end up duplicating the `HttpConfig` on the `Android Components` side. This is due to one `HttpConfig` setting the `viaduct` backend of the `Application Services` megazord, and one for our binary.
2. Having this as it's own repo would only work if we have a clean way to interact with `glean`
3. `rkv` worked really well for persistence.
4. We used `protobufs` in the prototype for the more complex data, the `glean-core` crate uses struct pointers, we'll need to discuss what the best way to pass in complex data is.
5. A quick comparison of the sizes of the release `aar`s (one built from the `main` branch, and another build with this) shows a `1MB` difference (`3761801` vs `2718455`), the most likely contributor to the size is the `protobuf` stuff we pull in. 
