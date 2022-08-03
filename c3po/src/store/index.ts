import Vue from "vue";
import Vuex from "vuex";
import appModule, {AppState} from "@/store/AppModule";

Vue.use(Vuex);

export interface State {
  app: AppState
}

export default new Vuex.Store<State>({
  modules: {
    app: appModule
  },
});
