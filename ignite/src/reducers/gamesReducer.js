const initState = {
    popular: [],
    newGames: [],
    upcoming: [],
}

const gameRreducer = (state = initState, action) => {

    switch (action.type) {

        case "FETCH_GAMES":
            return { ...state, popular: action.payload.popular }

        default:
            return { ...state }

    }

}

export default gameRreducer;