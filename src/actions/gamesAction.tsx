import axios from "axios";
import {
  popularGamesUrl,
  upComingGamesUrl,
  newGamesUrl,
  searchGameUrl,
} from "../api";

export const loadGames = () => async (dispatch: any) => {
  //fetch axios

  const popularData = await axios.get(popularGamesUrl());
  const upComingData = await axios.get(upComingGamesUrl());
  const newGamesData = await axios.get(newGamesUrl());

  dispatch({
    type: "FETCH_GAMES",
    payload: {
      popular: popularData.data.results,
      upcoming: upComingData.data.results,
      newGames: newGamesData.data.results,
    },
  });
};

export const fetchSearch = (game_name: any) => async (dispatch: any) => {
  const searchGames = await axios.get(searchGameUrl(game_name));

  dispatch({
    type: "FETCH_SEARCHED",
    payload: {
      searched: searchGames.data.results,
    },
  });
};
