# Rust-Tag

An Agent-based implementation of a game of tag or "It". Agents wander randomly around a 2D space. One of the agents is
the 'tagger' and when they come within a nearby radius of another player they tag the other player thus making them
the 'tagger'.

The simulation is a discrete-time stochastic agent-based model. The agents move around a discrete uniform space, each
taking up a cell's width.

## Requirements

* [Rust] (https://www.rust-lang.org/tools/install)

## Development and Usage

* To run the simulation, run either of the commands:
    * `cargo run --release` for an optimised release build
    * `cargo run` for a development build
* The benchmarks are ran with `cargo bench`

### Key Parameters

* In lieu of runtime arguments at the present there are a number of constants and parameters compiled into the binary.
  The main ones of interest are:
    * `main.rs`

      | Parameter | Functionality |
      | :---: | --- |
      | WORLD_WIDTH | Width of the simulation environment in cells |
      | WORLD_HEIGHT | Height of the simulation environment in cells |
      | NUM_AGENTS | Number of agents to run around the simulation |

    * `agents.rs`

      | Parameter | Functionality |
      | :---: | --- |
      | TAG_RADIUS | The distance (in cells) from which an agent can be tagged (inclusive)  |
      | AGENT_VIEW_DISTANCE | The distance (in cells) at which agents can see the tagger and vice versa (inclusive) |
      | MAX_VELOCITY | The maximum number of cells an agent can travel in one time-step |

    * `world_grid.rs`

      | Parameter | Functionality |
      | :---: | --- |
      | TAGGED_COLOUR | The 32-bit colour to draw the tagged agent in |
      | NORMAL_COLOUR | The 32-bit colour to draw all other agents in |
      | TAGGED_SCALE_INCREASE | The number of extra pixels to draw on each side of the tagged agent to help identify it at larger environment sizes |

## Disclaimer

This has only been tested on Windows, the visualisation library is supposedly cross-platform but such functionality is
unsupported for this project.