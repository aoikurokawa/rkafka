const initState = {
    popular: [],
    newGames: [],
    upcoming: [],
}

const gameRreducer = (state = initState, action) => {

    switch (action.type) {

        case "FETCH_GAMES":
            return { 
                ...state, 
                popular: action.payload.popular,
                upcoming: action.payload.upcoming, 
                newGames: action.payload.newGames,
             }

        default:
            return { ...state }

    }

}

export default gameRreducer;