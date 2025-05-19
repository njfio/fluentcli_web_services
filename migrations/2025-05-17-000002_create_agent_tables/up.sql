CREATE TABLE agent_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    conversation_id UUID NOT NULL REFERENCES conversations(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE agents (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id UUID NOT NULL REFERENCES agent_sessions(id),
    user_llm_config_id UUID NOT NULL REFERENCES user_llm_configs(id),
    role_name VARCHAR(255) NOT NULL,
    tool_config JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_agents_session_id ON agents(session_id);
CREATE INDEX idx_agents_user_llm_config_id ON agents(user_llm_config_id);

SELECT diesel_manage_updated_at('agent_sessions');
SELECT diesel_manage_updated_at('agents');
