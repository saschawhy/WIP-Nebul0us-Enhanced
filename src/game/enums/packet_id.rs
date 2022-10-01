pub enum PacketId {
    INVALID,
    CONNECT_RESULT_2,
    CONTROL,
    KEEP_ALIVE,
    INVALIDATE_CLIENT,
    START_GAME_INTERNAL,
    CONNECT_REQUEST,
    DISCONNECT,
    GAME_CHAT_MESSAGE,
    CLAN_CHAT_MESSAGE,
    JOIN_REQUEST,
    JOIN_RESULT,
    TTL_REFRESH_RESPONSE_INTERNAL,
    SHUTDOWN_NODE_INTERNAL,
    SET_GS_ADDR,
    CLIENT_PREFERENCES,
    SPECTATE_CHANGE,
    CLAN_WAR_LIST_REQUEST,
    CLAN_WAR_LIST_RESULT,
    CLAN_WAR_NOTIFICATION,
    TOP_SCORES,
    SERVER_SHUTDOWN_WARNING,
    GAME_UPDATE,
    GROUP_LOBBY_LIST_REQUEST,
    GROUP_LOBBY_LIST_RESULT,
    PUBLIC_CHAT_MESSAGE,
    ADMIN_INTERNAL,
    GROUP_LOBBY_CREATE_REQUEST,
    GROUP_LOBBY_CREATE_RESULT,
    GROUP_LOBBY_JOIN_REQUEST,
    GROUP_LOBBY_JOIN_RESULT,
    GROUP_LOBBY_UPDATE,
    GROUP_LOBBY_LEAVE,
    ARENA_LIST_REQUEST,
    CLIENT_PREFERENCES_INTERNAL,
    GAME_CRASH_INTERNAL,
    PRIVATE_CHAT_MESSAGE,
    ARENA_LEAVE_QUEUE_REQEUST,
    REMOVE_GAME_INTERNAL,
    GROUP_LOBBY_WARN,
    ENTER_GAME_REQUEST,
    ENTER_GAME_RESULT,
    PLAYER_SESSION_STATS_UPDATE_INTERNAL,
    PLAYER_WS_ACCOUNT_UPDATE_INTERNAL,
    ACCOUNT_STATUS_REQUEST,
    ACCOUNT_STATUS_RESULT,
    FRIEND_CHAT_MESAGE,
    CLIENT_STATUS_CHANGE_REQUEST,
    CLIENT_STATUS_CHANGE_RESULT,
    CLAN_WAR_CONTROL,
    CLAN_WAR_UPDATE,
    ARENA_LIST_RESULT,
    ADMIN_INTERNAL2,
    NODE_RESET_REQUEST_INTERNAL,
    CLAN_WAR_RESULT_INTERNAL,
    CLAN_WAR_FORFEIT_INTERNAL,
    SPECTATE_GAME_REQUEST,
    GET_PLAYER_STATS_INTERNAL,
    ARENA_QUEUE_REQUEST,
    ARENA_STATUS,
    ADMIN_INTERNAL3,
    ARENA_RESULT_INTERNAL,
    ADMIN_INTERNAL4,
    TEAM_ARENA_RESULT_INTERNAL,
    TEAM_ARENA_STATUS_RESULT,
    TEAM_ARENA_STATUS_REQUEST,
    TEAM_ARENA_LIST_REQUEST,
    TEAM_ARENA_LIST_RESULT,
    TEAM_ARENA_QUEUE_REQUEST,
    TEAM_ARENA_LEAVE_QUEUE_REQEUST,
    TEAM_ARENA_UPDATE,
    CLAN_HOUSE_UPDATE_INTERNAL,
    ADMIN_INTERNAL5,
    CLAN_HOUSE_UPDATE_INTERNAL2,
    NODE_CONNECT_REQUEST_INTERNAL,
    GAME_DATA,
    CHALLENGE,
    CHALLENGE_RESULT,
    FWD_TO_CLIENT_INTERNAL,
    TTL_REFRESH_REQUEST_INTERNAL,
    CONNECT_REQUEST_2,
    CONNECT_RESULT,
    ADMIN_INTERNAL6,
    CLAN_HOUSE_UPDATE_INTERNAL3,
    TOURNEY_LIST_REQUEST,
    TOURNEY_LIST_RESULT,
    TOURNEY_ACTION,
    TOURNEY_MATCH_RESULT_INTERNAL,
    TOURNEY_START_INTERNAL,
    TOURNEY_STATUS_UPDATE,
    ADMIN_INTERNAL7,
    MUTE_INTERNAL,
    JOINED_GAME_INTERNAL,
    CLAN_HOUSE_UPDATE_INTERNAL4,
    CLAN_HOUSE_CONFIG,
    INVITE,
    DESIRED_DUO_PARTNER,
    EMOTE_REQUEST,
    UDP_KEEPALIVE,
    GROUP_CHAT_CREATE_REQUEST,
    GROUP_CHAT_JOIN_REQUEST,
    GROUP_CHAT_LEAVE_REQUEST,
    GROUP_CHAT_RESULT,
    GROUP_CHAT_STATUS,
    GROUP_CHAT_MESSAGE,
    SESSION_STATS,
    ACCOLADE,
    VOICE_CONTROL,
    VOICE_DATA,
    MINIMAP_UPDATE,
    GAME_STOP_INTERNAL,
    BATTLE_ROYALE_ACTION,
    BATTLE_ROYALE_LIST_REQUEST,
    BATTLE_ROYALE_LIST_RESULT,
    BATTLE_ROYALE_STATUS_UPDATE,
    BATTLE_ROYALE_RESULT_INTERNAL,
    ADMIN_INTERNAL8,
    PING_MESSAGE,
    CONNECT_REQUEST_3,
    ARENA_CD_INTERNAL,
}
