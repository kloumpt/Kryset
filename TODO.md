# TODO
## Data
- [ ] Define the User type (Started)
- [ ] Define the Avatar type (Started)
- [ ] Define the World type (Started)
- [ ] Add some test elements (Started)
- [ ] Add Element saving functions
- [ ] Add Element loading functions

## Logic
- [ ] Listen to key inputs (Started)
- [ ] Allow avatar to explore the world (Started)
- [ ] Store the game state in a dedicated type

## Rendering
- [x] Display the world elemnts as ascii chars
- [x] Display the users avatar in the world
- [X] Store representation in a spécific type
- [X] Allow the display of composed representation
- [ ] Add Representations saving functions
- [ ] Add Representations loading functions

## Util
- [x] Manage text input
- [ ] Manage key input on a new thread

## Refactoring
- [x] Change User references avatar instead of avatar index in the world (fixed by using RC<RefCell<Avatar>> thanks to [desiringmachines](https://www.reddit.com/r/rust/comments/3rqrde/sharing_data_in_multiple_objects/cwrqf77))
- [x] Change World from referencing Avatars to referencing Elements

## Networking
- [ ] Add basic game hosting
- [ ] Add basic game joining
