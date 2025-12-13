## BEVY ECS

### Resources

### Components

### Systems

### Workflow diagram

```text
+-------------------+
|   Entity (ID)     |   <-- Just a generational index
+-------------------+
          |
          v
+-------------------+        +-------------------+
|   Archetype       | -----> |   Archetype       |   <-- Each archetype = unique set of components
|   (Table)         |        |   (Table)         |
+-------------------+        +-------------------+
          |                           |
          v                           v
+-------------------+        +-------------------+
| Component Column  |        | Component Column  |
| Position {x,y}    |        | Velocity {dx,dy} |
+-------------------+        +-------------------+
| Component Column  |        | Component Column  |
| Health {hp}       |        | Sprite {...}     |
+-------------------+        +-------------------+


Step 1: Spawn Entity
+-------------------+
| Entity: E1        |   <-- Just an ID
+-------------------+
        |
        v
+-------------------+
| Archetype A       |   <-- Empty table (no components)
+-------------------+
```

Step 2: Insert Position
commands.entity(E1).insert(Position {x:0, y:0})

Entity E1 migrates to Archetype B:
```text
+-------------------+
| Archetype B       |   <-- Table with Position column
+-------------------+
| Position {0,0}    |
+-------------------+
```


Step 3: Insert Velocity
commands.entity(E1).insert(Velocity {dx:1, dy:1})

Entity E1 migrates to Archetype C:
```text
+-------------------+
| Archetype C       |   <-- Table with Position + Velocity
+-------------------+
| Position {0,0}    |
| Velocity {1,1}    |
+-------------------+
```

Step 4: Remove Velocity
commands.entity(E1).remove::<Velocity>()

Entity E1 migrates back to Archetype B:
```text
+-------------------+
| Archetype B       |   <-- Table with Position only
+-------------------+
| Position {0,0}    |
+-------------------+
```

Step 5: Despawn Entity
commands.entity(E1).despawn()

Entity E1 is deleted, components removed from table.



🧩 Key Points
• Archetypes = tables defined by a unique set of component types.
• Migration = moving rows between tables when the component set changes.
• Insert/Remove triggers migration automatically.
• Systems don’t care about migration; they just query archetypes that match their component filters.

• Archetype = unique component set  
Each archetype corresponds to a specific combination of component types.  
Example:
	◦ Archetype A → {Position}
	◦ Archetype B → {Position, Velocity}
	◦ Archetype C → {Position, Velocity, Health}
• Archetype Table = storage for that set  
Each archetype has its own table with one column per component type.  
Entities in that archetype are rows in the table.
• Entities with identical component sets → same table  
If two entities both have Position and Velocity, they live in the same archetype table.  
Their Position components are stored contiguously in one column, and their Velocity components in another column.
Archetype B (Position + Velocity)

```text
+---------+----------------+----------------+
| Entity  | Position       | Velocity       |
+---------+----------------+----------------+
| E1      | {x:0, y:0}     | {dx:1, dy:1}   |
| E2      | {x:5, y:2}     | {dx:0, dy:-1}  |
| E3      | {x:-3, y:7}    | {dx:2, dy:0}   |
+---------+----------------+----------------+
```


• All entities (E1, E2, E3) with Position + Velocity reside in Archetype B’s table.
• Systems querying Query<(&Position, &Velocity)> iterate over this table directly, accessing contiguous slices of memory.
---
⚡ Why This Matters
• Cache efficiency → Components of the same type are stored contiguously, so iteration is fast.
• Parallelism → Systems can process archetype tables independently.
• Migration → Adding/removing a component moves the entity to a different archetype table.