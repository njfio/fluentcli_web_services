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

export interface Job {
  id: string;
  worker_type: string;
  config: string;
  pipeline_id: string;
  amber_id: string;
  status: string;
  state_file_content: string;
  createdAt: string;
  updatedAt: string;
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
  jobs: Job[];
  currentJob: Job | null;
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
    jobs: [],
    currentJob: null,
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
    setJobs(state: StudioState, jobs: Job[]) {
      state.jobs = jobs;
    },
    setCurrentJob(state: StudioState, job: Job) {
      state.currentJob = job;
    },
  },
  actions: {
    async fetchDockerFiles({ commit }) {
      try {
        const response = await apiClient.listDockerFiles();
        commit('setDockerFiles', response.data);
      } catch (error) {
        console.error('Error fetching docker files:', error);
        throw error;
      }
    },
    async fetchDockerFileById({ commit }, id: string) {
      try {
        const response = await apiClient.getDockerFile(id);
        commit('setCurrentDockerFile', response.data);
        return response.data;
      } catch (error) {
        console.error('Error fetching docker file:', error);
        throw error;
      }
    },
    async createDockerFile({ dispatch }, dockerFile: DockerFile) {
      try {
        await apiClient.createDockerFile(dockerFile);
        await dispatch('fetchDockerFiles');
      } catch (error) {
        console.error('Error creating docker file:', error);
        throw error;
      }
    },
    async updateDockerFile({ dispatch }, dockerFile: DockerFile) {
      try {
        await apiClient.updateDockerFile(dockerFile.id, dockerFile);
        await dispatch('fetchDockerFiles');
      } catch (error) {
        console.error('Error updating docker file:', error);
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
    async createPipeline({ dispatch }, pipeline: Pipeline) {
      try {
        await apiClient.createPipeline(pipeline);
        await dispatch('fetchPipelines');
      } catch (error) {
        console.error('Error creating pipeline:', error);
        throw error;
      }
    },
    async updatePipeline({ dispatch }, pipeline: Pipeline) {
      try {
        await apiClient.updatePipeline(pipeline.id, pipeline);
        await dispatch('fetchPipelines');
      } catch (error) {
        console.error('Error updating pipeline:', error);
        throw error;
      }
    },
    async fetchConfigurations({ commit }) {
      try {
        const response = await apiClient.listConfigurations();
        commit('setConfigurations', response.data);
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
    async createConfiguration({ dispatch }, configuration: StudioConfiguration) {
      try {
        await apiClient.createConfiguration(configuration);
        await dispatch('fetchConfigurations');
      } catch (error) {
        console.error('Error creating configuration:', error);
        throw error;
      }
    },
    async updateConfiguration({ dispatch }, configuration: StudioConfiguration) {
      try {
        await apiClient.updateConfiguration(configuration.id, configuration);
        await dispatch('fetchConfigurations');
      } catch (error) {
        console.error('Error updating configuration:', error);
        throw error;
      }
    },
    async fetchAmberStores({ commit }) {
      try {
        const response = await apiClient.listAmberStores();
        commit('setAmberStores', response.data);
      } catch (error) {
        console.error('Error fetching amber stores:', error);
        throw error;
      }
    },
    async fetchAmberStoreById({ commit }, id: string) {
      try {
        const response = await apiClient.getAmberStore(id);
        commit('setCurrentAmberStore', response.data);
        return response.data;
      } catch (error) {
        console.error('Error fetching amber store:', error);
        throw error;
      }
    },
    async createAmberStore({ dispatch }, amberStore: AmberStore) {
      try {
        await apiClient.createAmberStore(amberStore);
        await dispatch('fetchAmberStores');
      } catch (error) {
        console.error('Error creating amber store:', error);
        throw error;
      }
    },
    async updateAmberStore({ dispatch }, amberStore: AmberStore) {
      try {
        await apiClient.updateAmberStore(amberStore.id, amberStore);
        await dispatch('fetchAmberStores');
      } catch (error) {
        console.error('Error updating amber store:', error);
        throw error;
      }
    },
    async fetchJobs({ commit }) {
      try {
        const response = await apiClient.listJobs();
        commit('setJobs', response.data);
      } catch (error) {
        console.error('Error fetching jobs:', error);
        throw error;
      }
    },
    async fetchJobById({ commit }, id: string) {
      try {
        const response = await apiClient.getJob(id);
        commit('setCurrentJob', response.data);
        return response.data;
      } catch (error) {
        console.error('Error fetching job:', error);
        throw error;
      }
    },
    async saveJob({ dispatch }, job: Job) {
      try {
        if (job.id) {
          await apiClient.updateJob(job.id, job);
        } else {
          await apiClient.createJob(job);
        }
        await dispatch('fetchJobs');
      } catch (error) {
        console.error('Error saving job:', error);
        throw error;
      }
    },
    async deleteJob({ dispatch }, id: string) {
      try {
        await apiClient.deleteJob(id);
        await dispatch('fetchJobs');
      } catch (error) {
        console.error('Error deleting job:', error);
        throw error;
      }
    },
    async fetchAllData({ dispatch }) {
      await Promise.all([
        dispatch('fetchDockerFiles'),
        dispatch('fetchPipelines'),
        dispatch('fetchConfigurations'),
        dispatch('fetchAmberStores'),
        dispatch('fetchJobs'),
      ]);
    },
  },
  getters: {
    getDockerFiles: (state: StudioState) => state.dockerFiles,
    getCurrentDockerFile: (state: StudioState) => state.currentDockerFile,
    getPipelines: (state: StudioState) => state.pipelines,
    getCurrentPipeline: (state: StudioState) => state.currentPipeline,
    getConfigurations: (state: StudioState) => state.configurations,
    getCurrentConfiguration: (state: StudioState) => state.currentConfiguration,
    getAmberStores: (state: StudioState) => state.amberStores,
    getCurrentAmberStore: (state: StudioState) => state.currentAmberStore,
    getJobs: (state: StudioState) => state.jobs,
    getCurrentJob: (state: StudioState) => state.currentJob,
  },
};

export default studioModule;