import java.util.*

// requests & commands
data class FindPublicGame(val clientId: ClientId)

data class CreateGame(
    val clientId: ClientId,
    val visibility: Visibility,
    val gameId: GameId
)

/**
 * This request is issued when someone
 * wants to join a private game
 *
 * @param gameId    the game ID to join
 * @param clientId  the client ID of individual
 *                  who issued this request
 */
data class JoinPrivateGame(
    val gameId: GameId,
    val clientId: ClientId
)

// replies & events
/**
 * This event is issued when someone has created
 * a game and is waiting for their opponent to join.
 *
 * @param clientId The client ID of the individual
 *                  waiting.  This will be used
 *                  downstream to create the GameReady
 *                  event, which requires that both
 *                  players' clientIds are present
 */
data class WaitForOpponent(
    val gameId: GameId,
    val clientId: ClientId,
    val eventId: EventId = UUID.randomUUID()
)

data class GameReady(
    val gameId: GameId,
    val clients: Pair<ClientId, ClientId>,
    val eventId: EventId = UUID.randomUUID()
)

data class PrivateGameRejected(
    val gameId: GameId,
    val clientId: ClientId,
    val eventId: EventId = UUID.randomUUID()
)