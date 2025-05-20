-- First, check if the column exists to avoid errors
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM information_schema.columns
        WHERE table_name = 'messages'
        AND column_name = 'attachment_id'
    ) THEN
        -- Add the attachment_id column
        ALTER TABLE messages
        ADD COLUMN attachment_id UUID;
        
        -- Add foreign key constraint
        ALTER TABLE messages
        ADD CONSTRAINT fk_messages_attachment
        FOREIGN KEY (attachment_id)
        REFERENCES attachments(id);
    END IF;
    
    -- Check if the attachments column exists
    IF EXISTS (
        SELECT 1
        FROM information_schema.columns
        WHERE table_name = 'messages'
        AND column_name = 'attachments'
    ) THEN
        -- Drop the old attachments column
        ALTER TABLE messages
        DROP COLUMN attachments;
    END IF;
END
$$;
