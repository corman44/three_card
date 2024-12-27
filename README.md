# Summary

My Version of Three Card Game written in Rust using Bevy

# Game Rules

Seems similar to this: https://en.wikipedia.org/wiki/Shithead_(card_game)

### Start

Deal 3 cards face down to each player, Deal 3 cards face up, Deal 3 cards for each players hand (hence the name). Place the rest of the deck in the middle.

The person with the lowest card (excluding 2), goes first. If 2 players have the same low card then the younger player starts.

### Gameplay

Each player takes turns playing cards from their hand on the middle pile with the goal of running out of cards first 🥇. Each player must always have at least 3 cards in their hand (until the deck is gone). You must play a card that is equal or higher than the top card on the pile. You may play up to 4 cards at once (given you have 4 of a single card). If a player can't play they must pickup the pile.

When the deck is empty and players have no cards in their hands, they must play from the face-up cards on the cards on the table. Once the face-up cards are played the player and it is their turn, the player must randomly choose a facedown card to play.

**Special Rules**:

1. The 2 card restarts the pile. The player that played the 2 may pickup (up to 3) and it is their turn.
2. The 10 card blows up the pile. The player that played the 10 may pickup (up to 3) and it is their turn.
3. If 4 of a single card are played consecutively, the pile is blown up and the player who played the last card may pickup and play their turn again.

# Inner Workings

- Bevy Game Engine: https://bevyengine.org/
- https://github.com/johanhelsing/matchbox
- https://johanhelsing.studio/posts/extreme-bevy

### Client Signaling

- Number of deck cards picked up uint: [u2]
- Card values played uint: [u4, u4, u4, u4]
- Pile picked up bool: [u1]
- UI for Table Cards bool: [u3]

### Startup Process

1. Initialize States, Resources, Rollback
2. Start Matchbox Socket and wait for other players
3. After connection to other player, calculate seed based on both PeerIds
4. Shuffle deck based on Seed and deal to each player
