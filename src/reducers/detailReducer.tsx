const initialState = {
  game: { platforms: [] },
  screen: { results: [] },
  isLoading: true,
  getDetail: false,
};

const detailReducer = (state = initialState, action) => {
  switch (action.type) {
    case "GET_DETAIL":
      return {
        ...state,
        game: action.payload.game,
        screen: action.payload.screen,
        isLoading: false,
        getDetail: true,
      };

    case "LOADING_DETAIL":
      return {
        ...state, 
        isLoading: true,
        getDetail: false,
      }

    case "CLEAR_DETAIL":
      return {
        ...state, 
        game: { platforms: [] },
        screen: { results: [] },
        getDetail: false,
      }
      
    default:
      return {
        ...state,
      };
  }
};

export default detailReducer;
