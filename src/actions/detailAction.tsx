import axios from "axios";
import { gameDetailsUrl, gameScreenshotUrl } from "../api";

export const loadDetail = (id: any) => async (dispatch: any) => {
  const detailData = await axios.get(gameDetailsUrl(id));
  const screeShotData = await axios.get(gameScreenshotUrl(id));

  dispatch({
    type: "LOADING_DETAIL",
  });

  dispatch({
    type: "GET_DETAIL",
    payload: {
      game: detailData.data,
      screen: screeShotData.data,
    },
  });
};


