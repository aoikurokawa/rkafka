import axios from "axios";
import { popularGamesUrl } from '../api';

export const loadGames = () => async (dispatch) => {
    //fetch axios
    const popularData = await axios.get(popularGamesUrl());
    dispatch({
        type: "FETCH_GAMES", 
        payload: {
            popular: popularData.data.results, 
        }
    })
} 

