# TODO
## Data
- [ ] Define the User type (Started)
- [ ] Define the Avatar type (Started)
- [ ] Define the World type (Started)

## Logic
- [ ] Listen to key inputs (Started)
- [ ] Allow avatar to explore the world (Started)

## Rendering
- [ ] Display the world as ascii chars
- [ ] Display the users avatar in the world (Started)

## Util
- [x] Manage text input

## Refactoring
- [x] Change User references avatar instead of avatar index in the world (fixed by using RC<RefCell<Avatar>> thanks to [desiringmachines](https://www.reddit.com/r/rust/comments/3rqrde/sharing_data_in_multiple_objects/cwrqf77))
- [ ] Change World from referencing Avatars to referencing Elements
