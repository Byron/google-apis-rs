*Youtube* is a library written in Rust to help interacting with your youtube account.
For now, all functionality is geared towards allowing interruptible video uploads
and adjustments of video meta-data.

The library works using the builder pattern. If builders are instantiated, you will need to 
provide the minimal information right off the bat. Further calls to the builder allow 
to configure it. Some configuration will be in the form of callbacks, which allows you to 
control internal loops or behaviour.

It's the goal of each builder to maximize the chances of a successful result, and it will 
provide enough callbacks to be resilient against network errors, and authorization failures 
which require the token to be refreshed.

You will need authorization to perform most of the operations implemented here - it can be obtained
and handled using the [yup-oauth2 library][oauth].

[oauth]: [https://crates.io/crates/yup-oauth2]