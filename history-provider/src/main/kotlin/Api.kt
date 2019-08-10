import com.fasterxml.jackson.annotation.JsonIgnoreProperties
import com.fasterxml.jackson.annotation.JsonTypeInfo
import java.time.Instant

data class ProvideHistoryCommand(val gameId: GameId, val reqId: ReqId)

/**
 * An ordered list of moves (FIFO).
 * Jackson annotations required so that gateway can deserialize this.
 *
 * @property gameId         the game associated with this history
 * @property replyTo        the original request ID which triggered the
 *                          generation of this event
 * @property eventId        an ID unique to this particular event
 * @property moves          an ordered list of moves which have occurred;
 *                          first move is head of the list
 * @property epochMillis    a timestamp for this event
 */
@JsonTypeInfo(
    use = JsonTypeInfo.Id.NAME,
    include = JsonTypeInfo.As.PROPERTY,
    property = "type"
)
@JsonIgnoreProperties(value = ["type"]) // madness, we have to ignore it on deser
data class HistoryProvided( // THIS NAME MUST NOT CHANGE -- gateway depends
// on JSON
    val gameId: GameId,
    val replyTo: ReqId,
    val eventId: EventId,
    val moves: List<Move>,
    val epochMillis: Long = Instant.now().toEpochMilli()
)