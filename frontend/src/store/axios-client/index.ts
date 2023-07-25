import create, { GetState, SetState } from "zustand";
import { AxiosClientStore } from "../../types/store/axios-client";
import axiosClientActions from "./actions";
import state from "./state";

const axiosClient = (set: SetState<AxiosClientStore>, get: GetState<AxiosClientStore>) => ({
    ...state,
    ...axiosClientActions(set, get),
});

const useAxiosClient = create<AxiosClientStore>(axiosClient);

export default useAxiosClient;
