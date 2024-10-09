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
  status: string;
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
  created_at: string;
  updated_at: string;
  started_at: string | null;
  completed_at: string | null;
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
    setDockerFiles(state, dockerFiles) {
      state.dockerFiles = dockerFiles;
    },
    setCurrentDockerFile(state, dockerFile) {
      state.currentDockerFile = dockerFile;
    },
    setPipelines(state, pipelines) {
      state.pipelines = pipelines;
    },
    setCurrentPipeline(state, pipeline) {
      state.currentPipeline = pipeline;
    },
    setConfigurations(state, configurations) {
      state.configurations = configurations;
    },
    setCurrentConfiguration(state, configuration) {
      state.currentConfiguration = configuration;
    },
    setAmberStores(state, amberStores) {
      state.amberStores = amberStores;
    },
    setCurrentAmberStore(state, amberStore) {
      state.currentAmberStore = amberStore;
    },
    setJobs(state, jobs) {
      state.jobs = jobs;
    },
    setCurrentJob(state, job) {
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
    async fetchPipelines({ commit }) {
      try {
        const response = await apiClient.listPipelines();
        commit('setPipelines', response.data);
      } catch (error) {
        console.error('Error fetching pipelines:', error);
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
    async fetchAmberStores({ commit }) {
      try {
        const response = await apiClient.listAmberStores();
        commit('setAmberStores', response.data);
      } catch (error) {
        console.error('Error fetching amber stores:', error);
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
    async fetchAllData({ dispatch }) {
      try {
        await Promise.all([
          dispatch('fetchDockerFiles'),
          dispatch('fetchPipelines'),
          dispatch('fetchConfigurations'),
          dispatch('fetchAmberStores'),
          dispatch('fetchJobs'),
        ]);
      } catch (error) {
        console.error('Error fetching all data:', error);
        throw error;
      }
    },
    async fetchJobById({ commit }, jobId: string) {
      try {
        const response = await apiClient.getJob(jobId);
        commit('setCurrentJob', response.data);
      } catch (error) {
        console.error('Error fetching job by ID:', error);
        throw error;
      }
    },
    async fetchDockerFileById({ commit }, id: string) {
      try {
        const response = await apiClient.getDockerFile(id);
        commit('setCurrentDockerFile', response.data);
      } catch (error) {
        console.error('Error fetching docker file by ID:', error);
        throw error;
      }
    },
    async fetchConfigurationById({ commit }, id: string) {
      try {
        const response = await apiClient.getConfiguration(id);
        commit('setCurrentConfiguration', response.data);
      } catch (error) {
        console.error('Error fetching configuration by ID:', error);
        throw error;
      }
    },
    async fetchAmberStoreById({ commit }, id: string) {
      try {
        const response = await apiClient.getAmberStore(id);
        commit('setCurrentAmberStore', response.data);
      } catch (error) {
        console.error('Error fetching amber store by ID:', error);
        throw error;
      }
    },
    async fetchPipelineById({ commit }, id: string) {
      try {
        const response = await apiClient.getPipeline(id);
        commit('setCurrentPipeline', response.data);
      } catch (error) {
        console.error('Error fetching pipeline by ID:', error);
        throw error;
      }
    },
    async updatePipeline({ commit }, pipeline: Pipeline) {
      try {
        const response = await apiClient.updatePipeline(pipeline.id, pipeline);
        commit('setCurrentPipeline', response.data);
      } catch (error) {
        console.error('Error updating pipeline:', error);
        throw error;
      }
    },
    async updateConfiguration({ commit }, configuration: StudioConfiguration) {
      try {
        const response = await apiClient.updateConfiguration(configuration.id, configuration);
        commit('setCurrentConfiguration', response.data);
      } catch (error) {
        console.error('Error updating configuration:', error);
        throw error;
      }
    },
    async updateDockerFile({ commit }, dockerFile: DockerFile) {
      try {
        const response = await apiClient.updateDockerFile(dockerFile.id, dockerFile);
        commit('setCurrentDockerFile', response.data);
      } catch (error) {
        console.error('Error updating docker file:', error);
        throw error;
      }
    },
    async updateAmberStore({ commit }, amberStore: AmberStore) {
      try {
        const response = await apiClient.updateAmberStore(amberStore.id, amberStore);
        commit('setCurrentAmberStore', response.data);
      } catch (error) {
        console.error('Error updating amber store:', error);
        throw error;
      }
    },
    async startJob({ commit, dispatch }, jobId: string) {
      try {
        const response = await apiClient.startJob(jobId);
        commit('setCurrentJob', response.data);
        // Refresh the jobs list after starting the job
        await dispatch('fetchJobs');
      } catch (error) {
        console.error('Error starting job:', error);
        throw error;
      }
    },
  },
  getters: {
    getDockerFiles: (state) => state.dockerFiles,
    getCurrentDockerFile: (state) => state.currentDockerFile,
    getPipelines: (state) => state.pipelines,
    getCurrentPipeline: (state) => state.currentPipeline,
    getConfigurations: (state) => state.configurations,
    getCurrentConfiguration: (state) => state.currentConfiguration,
    getAmberStores: (state) => state.amberStores,
    getCurrentAmberStore: (state) => state.currentAmberStore,
    getJobs: (state) => state.jobs,
    getCurrentJob: (state) => state.currentJob,
  },
};

export default studioModule;