const initState = {
  popular: [],
  newGames: [],
  upcoming: [],
  isloaded: true,
  searched: [],
};

const gameRreducer = (state = initState, action) => {
  switch (action.type) {
    case "FETCH_GAMES":
      return {
        ...state,
        popular: action.payload.popular,
        upcoming: action.payload.upcoming,
        newGames: action.payload.newGames,
        isloaded: false,
      };

    case "FETCH_SEARCHED":
      return {
        ...state,
        searched: action.payload.searched,
      };

    case "CLEAR_SEARCHED":
      return {
        ...state,
        searched: [],
      };

    default:
      return { ...state };
  }
};

export default gameRreducer;
