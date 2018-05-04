# Space Invaders project for MLP subject

A project for developing space invaders with Rust in both OOP and Functional paradigms.

## Running Docker environment

First, you need to install docker on your machine, then:

- ´´´git clone https://github.com/lanahra/space-invaders.git´´´

- ´´´cd space-invaders´´´

- ´´´docker build . -t space-invaders´´´

- ´´´docker -v %cd%:/usr/projects/space-invaders -it space-invaders bash´´´

// TODO Configure docker to run with -e DISPLAY for running GUI part of application