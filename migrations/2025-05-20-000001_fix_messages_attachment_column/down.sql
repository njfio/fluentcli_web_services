-- First, check if the attachments column exists to avoid errors
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM information_schema.columns
        WHERE table_name = 'messages'
        AND column_name = 'attachments'
    ) THEN
        -- Add the attachments column back
        ALTER TABLE messages
        ADD COLUMN attachments UUID[] DEFAULT '{}';
    END IF;
    
    -- Check if the attachment_id column exists
    IF EXISTS (
        SELECT 1
        FROM information_schema.columns
        WHERE table_name = 'messages'
        AND column_name = 'attachment_id'
    ) THEN
        -- Drop the foreign key constraint first
        ALTER TABLE messages
        DROP CONSTRAINT IF EXISTS fk_messages_attachment;
        
        -- Drop the attachment_id column
        ALTER TABLE messages
        DROP COLUMN attachment_id;
    END IF;
END
$$;
