# TODO
## Data
- [ ] Define the User type (Started)
- [ ] Define the Avatar type (Started)
- [ ] Define the World type (Started)
- [ ] Add some test elements (Started)

## Logic
- [ ] Listen to key inputs (Started)
- [ ] Allow avatar to explore the world (Started)
- [ ] Store the game state in a dedicated type

## Rendering
- [x] Display the world elemnts as ascii chars
- [x] Display the users avatar in the world
- [ ] Store representation in a spécific type
- [ ] Allow the display of composed representation

## Util
- [x] Manage text input
- [ ] Manage key input on a new thread

## Refactoring
- [x] Change User references avatar instead of avatar index in the world (fixed by using RC<RefCell<Avatar>> thanks to [desiringmachines](https://www.reddit.com/r/rust/comments/3rqrde/sharing_data_in_multiple_objects/cwrqf77))
- [x] Change World from referencing Avatars to referencing Elements
