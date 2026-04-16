# Prisma TypeScript Client Reference

This document serves as a comprehensive reference for the methods and arguments available in the official Prisma TypeScript Client.

## Model Methods

### Read Operations

#### `findMany`
Returns a list of records.
- **Args:**
  - `where`: Filter criteria.
  - `orderBy`: Sort order.
  - `cursor`: Pointer to a record for pagination.
  - `take`: Number of records to return.
  - `skip`: Number of records to skip.
  - `distinct`: Filter by unique field values.
  - `select`: Choose specific fields to return.
  - `include`: Include related records.

#### `findUnique` / `findUniqueOrThrow`
Returns a single record by a unique identifier.
- **Args:**
  - `where`: Must contain a unique field (e.g., `@id` or `@unique`).
  - `select`: Choose specific fields.
  - `include`: Include related records.

#### `findFirst` / `findFirstOrThrow`
Returns the first record matching criteria.
- **Args:**
  - `where`, `orderBy`, `cursor`, `take`, `skip`, `distinct`, `select`, `include`.

### Write Operations

#### `create`
Creates a new record.
- **Args:**
  - `data`: Field values for the new record.
  - `select`, `include`.

#### `update`
Updates an existing record by unique identifier.
- **Args:**
  - `where`: Unique identifier.
  - `data`: New field values.
  - `select`, `include`.

#### `upsert`
Updates an existing record or creates a new one if it doesn't exist.
- **Args:**
  - `where`: Unique identifier.
  - `update`: Values for update.
  - `create`: Values for creation.
  - `select`, `include`.

#### `delete`
Deletes a record by unique identifier.
- **Args:**
  - `where`: Unique identifier.
  - `select`, `include`.

#### `createMany`
Creates multiple records in a single query.
- **Args:**
  - `data`: Array of field values.
  - `skipDuplicates`: Boolean.

#### `updateMany`
Updates multiple records matching criteria.
- **Args:**
  - `where`: Filter criteria.
  - `data`: New field values.

#### `deleteMany`
Deletes multiple records matching criteria.
- **Args:**
  - `where`: Filter criteria.

### Aggregation Operations

#### `count`
Returns the number of records matching criteria.
- **Args:**
  - `where`, `orderBy`, `cursor`, `take`, `skip`, `select` (for counting specific fields).

#### `aggregate`
Performs aggregations (sum, avg, etc.) on fields.
- **Args:**
  - `where`, `orderBy`, `cursor`, `take`, `skip`, `_count`, `_sum`, `_avg`, `_min`, `_max`.

#### `groupBy`
Groups records by fields and performs aggregations.
- **Args:**
  - `by`: Fields to group by.
  - `where`, `having`, `take`, `skip`, `orderBy`, `_count`, `_sum`, etc.

## Global Methods

- `$connect()`: Connect to database.
- `$disconnect()`: Disconnect from database.
- `$transaction()`: Execute multiple queries in a transaction.
- `$queryRaw`: Execute raw SQL/NoSQL query (returns records).
- `$executeRaw`: Execute raw SQL/NoSQL command (returns count).
- `$use()`: Middleware support.
