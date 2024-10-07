import { Module } from 'vuex';
import { RootState } from '../types';
import apiClient from '@/services/apiClient';

export interface DockerFile {
  id: string;
  name: string;
  description: string;
  content: string;
  createdAt: string;
  updatedAt: string;
}

export interface Pipeline {
  id: string;
  name: string;
  description: string;
  createdAt: string;
  lastModified: string;
}

export interface StudioConfiguration {
  id: string;
  name: string;
  description: string;
  type: string;
  status: string;
  createdAt: string;
  updatedAt: string;
}

export interface AmberStore {
  id: string;
  name: string;
  description: string;
  data: string;
  secure_key_hash: string;
  createdAt: string;
  lastModified: string;
}

export interface StudioState {
  dockerFiles: DockerFile[];
  currentDockerFile: DockerFile | null;
  pipelines: Pipeline[];
  currentPipeline: Pipeline | null;
  configurations: StudioConfiguration[];
  currentConfiguration: StudioConfiguration | null;
  amberStores: AmberStore[];
  currentAmberStore: AmberStore | null;
}

const studioModule: Module<StudioState, RootState> = {
  namespaced: true,
  state: {
    dockerFiles: [],
    currentDockerFile: null,
    pipelines: [],
    currentPipeline: null,
    configurations: [],
    currentConfiguration: null,
    amberStores: [],
    currentAmberStore: null,
  },
  mutations: {
    setDockerFiles(state: StudioState, dockerFiles: DockerFile[]) {
      state.dockerFiles = dockerFiles;
    },
    setCurrentDockerFile(state: StudioState, dockerFile: DockerFile) {
      state.currentDockerFile = dockerFile;
    },
    setPipelines(state: StudioState, pipelines: Pipeline[]) {
      state.pipelines = pipelines;
    },
    setCurrentPipeline(state: StudioState, pipeline: Pipeline) {
      state.currentPipeline = pipeline;
    },
    setConfigurations(state: StudioState, configurations: StudioConfiguration[]) {
      state.configurations = configurations;
    },
    setCurrentConfiguration(state: StudioState, configuration: StudioConfiguration) {
      state.currentConfiguration = configuration;
    },
    setAmberStores(state: StudioState, amberStores: AmberStore[]) {
      state.amberStores = amberStores;
    },
    setCurrentAmberStore(state: StudioState, amberStore: AmberStore) {
      state.currentAmberStore = amberStore;
    },
  },
  actions: {
    async fetchDockerFiles({ commit }) {
      try {
        const response = await apiClient.listDockerFiles();
        commit('setDockerFiles', response.data);
      } catch (error) {
        console.error('Error fetching Docker files:', error);
        throw error;
      }
    },
    async fetchDockerFileById({ commit }, id: string) {
      try {
        const response = await apiClient.getDockerFile(id);
        commit('setCurrentDockerFile', response.data);
        return response.data;
      } catch (error) {
        console.error('Error fetching Docker file:', error);
        throw error;
      }
    },
    async saveDockerFile({ dispatch }, dockerFile: DockerFile) {
      try {
        if (dockerFile.id) {
          await apiClient.updateDockerFile(dockerFile.id, dockerFile);
        } else {
          await apiClient.createDockerFile(dockerFile);
        }
        await dispatch('fetchDockerFiles');
      } catch (error) {
        console.error('Error saving Docker file:', error);
        throw error;
      }
    },
    async deleteDockerFile({ dispatch }, id: string) {
      try {
        await apiClient.deleteDockerFile(id);
        await dispatch('fetchDockerFiles');
      } catch (error) {
        console.error('Error deleting Docker file:', error);
        throw error;
      }
    },
    async fetchPipelines({ commit }) {
      try {
        const response = await apiClient.listPipelines();
        commit('setPipelines', response.data);
      } catch (error) {
        console.error('Error fetching pipelines:', error);
        throw error;
      }
    },
    async fetchPipelineById({ commit }, id: string) {
      try {
        const response = await apiClient.getPipeline(id);
        commit('setCurrentPipeline', response.data);
        return response.data;
      } catch (error) {
        console.error('Error fetching pipeline:', error);
        throw error;
      }
    },
    async savePipeline({ dispatch }, pipeline: Pipeline) {
      try {
        if (pipeline.id) {
          await apiClient.updatePipeline(pipeline.id, pipeline);
        } else {
          await apiClient.createPipeline(pipeline);
        }
        await dispatch('fetchPipelines');
      } catch (error) {
        console.error('Error saving pipeline:', error);
        throw error;
      }
    },
    async deletePipeline({ dispatch }, id: string) {
      try {
        await apiClient.deletePipeline(id);
        await dispatch('fetchPipelines');
      } catch (error) {
        console.error('Error deleting pipeline:', error);
        throw error;
      }
    },
    async fetchConfigurations({ commit }) {
      console.log('Fetching configurations from Vuex action');
      try {
        const response = await apiClient.listConfigurations();
        console.log('API response for configurations:', response);
        commit('setConfigurations', response.data);
        console.log('Configurations committed to state:', response.data);
      } catch (error) {
        console.error('Error fetching configurations:', error);
        throw error;
      }
    },
    async fetchConfigurationById({ commit }, id: string) {
      try {
        const response = await apiClient.getConfiguration(id);
        commit('setCurrentConfiguration', response.data);
        return response.data;
      } catch (error) {
        console.error('Error fetching configuration:', error);
        throw error;
      }
    },
    async saveConfiguration({ dispatch }, configuration: StudioConfiguration) {
      try {
        if (configuration.id) {
          await apiClient.updateConfiguration(configuration.id, configuration);
        } else {
          await apiClient.createConfiguration(configuration);
        }
        await dispatch('fetchConfigurations');
      } catch (error) {
        console.error('Error saving configuration:', error);
        throw error;
      }
    },
    async deleteConfiguration({ dispatch }, id: string) {
      try {
        await apiClient.deleteConfiguration(id);
        await dispatch('fetchConfigurations');
      } catch (error) {
        console.error('Error deleting configuration:', error);
        throw error;
      }
    },
    async fetchAmberStores({ commit }) {
      console.log('Fetching Amber Stores...');
      try {
        const response = await apiClient.listAmberStores();
        console.log('Amber Stores API response:', response);
        commit('setAmberStores', response.data);
        console.log('Amber Stores committed to state:', response.data);
      } catch (error: any) {
        console.error('Error fetching Amber Stores:', error);
        if (error.response) {
          console.error('Error response:', error.response.data);
          console.error('Error status:', error.response.status);
          console.error('Error headers:', error.response.headers);
        } else if (error.request) {
          console.error('Error request:', error.request);
        } else {
          console.error('Error message:', error.message);
        }
        throw error;
      }
    },
    async fetchAmberStoreById({ commit }, id: string) {
      try {
        const response = await apiClient.getAmberStore(id);
        commit('setCurrentAmberStore', response.data);
        return response.data;
      } catch (error) {
        console.error('Error fetching Amber Store:', error);
        throw error;
      }
    },
    async saveAmberStore({ dispatch }, amberStore: AmberStore) {
      try {
        if (amberStore.id) {
          await apiClient.updateAmberStore(amberStore.id, amberStore);
        } else {
          await apiClient.createAmberStore(amberStore);
        }
        await dispatch('fetchAmberStores');
      } catch (error) {
        console.error('Error saving Amber Store:', error);
        throw error;
      }
    },
    async deleteAmberStore({ dispatch }, id: string) {
      try {
        await apiClient.deleteAmberStore(id);
        await dispatch('fetchAmberStores');
      } catch (error) {
        console.error('Error deleting Amber Store:', error);
        throw error;
      }
    },
  },
  getters: {
    getDockerFiles: (state: StudioState) => state.dockerFiles,
    getCurrentDockerFile: (state: StudioState) => state.currentDockerFile,
    getPipelines: (state: StudioState) => state.pipelines,
    getCurrentPipeline: (state: StudioState) => state.currentPipeline,
    getConfigurations: (state: StudioState) => {
      console.log('Getter called for configurations:', state.configurations);
      return state.configurations;
    },
    getCurrentConfiguration: (state: StudioState) => state.currentConfiguration,
    getAmberStores: (state: StudioState) => state.amberStores,
    getCurrentAmberStore: (state: StudioState) => state.currentAmberStore,
  },
};

export default studioModule;