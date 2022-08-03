import EmpireData from "@/models/empire_data";
import {ActionContext} from "vuex";
import BountyHunter from "@/models/bounty_hunter";
import axios from "axios";

export interface AppState {
    empire_data: EmpireData,
    simulation_result: number
}

export default {
    namespaced: true,
    state: (): AppState => ({
       empire_data: {countdown: 0, bounty_hunters: []},
       simulation_result: 0
    }),
    mutations: {
        setEmpireData: function(state: AppState, payload: EmpireData): void {
            state.empire_data = payload;
        },
        setCountdown: function(state: AppState, payload: number): void {
            state.empire_data.countdown = payload;
        },
        setResult: function(state: AppState, payload: number): void {
            state.simulation_result = payload
        },
        addBountyHunter: function(state: AppState, payload: BountyHunter): void {
            state.empire_data.bounty_hunters.push(payload);
        }
    },
    actions: {
        setCountdown: function (context: any, countdown: number) {
            context.commit("setCountdown", countdown)
        },
        addBountyHunter: function(context: any, bh: BountyHunter) {
            context.commit("addBountyHunter", bh);
        },
        simulate: function(context: any) {
            axios.post("http://localhost:8000/sim", context.state.empire_data).then(r => context.commit("setResult", r.data));
        }
    },
    getters: {
        getEmpireData(state: AppState) {
            return state.empire_data;
        },
        getSimulationResult(state: AppState) {
            return state.simulation_result;
        }
    }
}