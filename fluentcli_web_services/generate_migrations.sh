
#!/bin/bash
set -e

# Use environment variables for database connection
export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${PG_HOST}:5432/${POSTGRES_DB}"

# Function to convert Rust types to SQL types
rust_to_sql_type() {
    local type=$1
    case $type in
        "Uuid") echo "UUID";;
        "Varchar") echo "VARCHAR(255)";;
        "Text") echo "TEXT";;
        "Timestamptz") echo "TIMESTAMP WITH TIME ZONE";;
        "Bool") echo "BOOLEAN";;
        "Jsonb") echo "JSONB";;
        *) 
            if [[ $type == Nullable* ]]; then
                inner_type=$(echo $type | sed -E 's/Nullable<(.*)>/\1/')
                echo "$(rust_to_sql_type $inner_type)"
            else
                echo "UNKNOWN_TYPE"
            fi
            ;;
    esac
}
# Read the migration.rs file and generate migrations
while IFS= read -r line
do
    if [[ $line =~ diesel::table\!\ *\{\ *([a-z_]+) ]]; then
        table_name=${BASH_REMATCH[1]}
        migration_name="create_${table_name}"
        
        # Check if migration already exists
        if ! ls -d migrations/*_${migration_name} > /dev/null 2>&1; then
            diesel migration generate $migration_name

            # Get the latest migration directory
            migration_dir=$(ls -d migrations/*_${migration_name} | tail -n 1)
            up_sql="${migration_dir}/up.sql"
            down_sql="${migration_dir}/down.sql"

            echo "CREATE TABLE ${table_name} (" > "$up_sql"
            echo "DROP TABLE ${table_name};" > "$down_sql"

            # Read table definition
            while IFS= read -r def_line
            do
                if [[ $def_line =~ ([a-z_]+)\ *-\>\ *([A-Za-z<>]+), ]]; then
                    column_name=${BASH_REMATCH[1]}
                    rust_type=${BASH_REMATCH[2]}
                    sql_type=$(rust_to_sql_type "$rust_type")
                    
                    if [[ $column_name == "id" ]]; then
                        echo "    $column_name $sql_type PRIMARY KEY," >> "$up_sql"
                    elif [[ $rust_type == Nullable* ]]; then
                        echo "    $column_name $sql_type," >> "$up_sql"
                    else
                        echo "    $column_name $sql_type NOT NULL," >> "$up_sql"
                    fi
                elif [[ $def_line == "}" ]]; then
                    break
                fi
            done
            # Remove the trailing comma and close the parenthesis
            sed -i '$ s/,$//' "$up_sql"
            echo ");" >> "$up_sql"
            echo "Migration generated for table: ${table_name}"
        else
            echo "Migration already exists for table: ${table_name}"
        fi
    fi
done < migrations/migration.rs


# Run migrations and update schema
diesel setup
diesel migration run
diesel print-schema > src/schema.rs.new
if [ -s src/schema.rs.new ]; then
    if [ -s src/schema.rs ]; then
        # Merge the new schema with the existing one
        cat src/schema.rs.new >> src/schema.rs
        sort -u src/schema.rs -o src/schema.rs
    else
        mv src/schema.rs.new src/schema.rs
    fi
else
    rm src/schema.rs.new
fi

echo "Migrations generated and applied successfully. Schema updated."
