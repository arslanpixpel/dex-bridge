import { AxiosClient as Client } from "../../../api-query/__generated__/AxiosClient";

interface AxiosClientState {
    client: Client | undefined;
}

export interface AxiosClientActions {
    getClient: () => Promise<Client | undefined>;
}

export type AxiosClientStore = AxiosClientState & AxiosClientActions;
