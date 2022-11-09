File Structure:

```bash
src
├── context <- Bounded Context
│   ├── common <- Common Functionality
│   │   ├── domain <- Common 'Base' Domain Entities (e.g. Aggregate, Event)
│   │   │   ├── entity
│   │   │   │   ├── aggregate.rs
│   │   │   │   ├── event.rs
│   │   │   │   └── mod.rs
│   │   │   ├── machine <- Library providing state machine functionality
│   │   │   │   └── mod.rs
│   │   │   ├── mod.rs
│   │   │   └── ports <- Common Interfaces
│   │   │       ├── mod.rs
│   │   │       └── outbound <- Outbound dependencies within our system
│   │   │           ├── event_bus.rs <- Event Transfer
│   │   │           ├── event_repository.rs <- Event Persistance
│   │   │           └── mod.rs
│   │   ├── infrastructure <- Common Implementations
│   │   │   ├── adapters
│   │   │   │   ├── mod.rs
│   │   │   │   └── secondary
│   │   │   │       ├── eventbus
│   │   │   │       │   ├── channel.rs <- An EentBus that is just a MPMC channel
│   │   │   │       │   └── mod.rs
│   │   │   │       ├── mod.rs
│   │   │   │       └── storage
│   │   │   │           ├── mod.rs
│   │   │   │           └── sqlite.rs <- A SQLite Database Adapter
│   │   │   ├── dtos <- Common Data Transfer Objects
│   │   │   │   ├── mod.rs
│   │   │   │   └── storage
│   │   │   │       ├── mod.rs
│   │   │   │       └── sql.rs <- Snapshot and EventEnvlope Serialization/Deserialize SQLite Row
│   │   │   └── mod.rs
│   │   └── mod.rs
│   ├── mod.rs
│   └── prescription <- Main Business Area
│       ├── application
│       │   ├── mod.rs
│       │   ├── ports
│       │   │   ├── inbound <- Interfaces for our public API
│       │   │   │   ├── create_prescription.rs
│       │   │   │   ├── get_events.rs
│       │   │   │   ├── mod.rs
│       │   │   │   ├── send_event.rs
│       │   │   │   └── update_prescription.rs
│       │   │   ├── mod.rs
│       │   │   └── outbound <- Unused, 3rd party service dependency interfaces
│       │   │       ├── mod.rs
│       │   │       └── prescription.rs
│       │   └── service <- Map inbound ports -> outbound port(s)
│       │       ├── mod.rs
│       │       ├── outbox.rs
│       │       └── prescription.rs
│       ├── config <- Configuration
│       │   └── mod.rs
│       ├── domain
│       │   ├── entity
│       │   │   ├── aggregate.rs <- PrescriptionAggregate
│       │   │   ├── command.rs <- PrescriptionCommand
│       │   │   ├── error.rs
│       │   │   ├── event.rs <- PrescriptionEvents
│       │   │   └── mod.rs
│       │   ├── machine <- State Machine that governs Prescription logic
│       │   │   ├── context.rs
│       │   │   ├── mod.rs
│       │   │   └── states
│       │   │       ├── created.rs
│       │   │       ├── mod.rs
│       │   │       └── new.rs
│       │   └── mod.rs
│       ├── infrastructure
│       │   ├── adapters
│       │   │   ├── mod.rs
│       │   │   ├── primary
│       │   │   │   ├── mod.rs
│       │   │   │   └── rest.rs <- Inbound REST Interface
│       │   │   └── secondary
│       │   │       ├── mod.rs
│       │   │       └── sqlite.rs <- PrescriptionRepository SQLite
│       │   ├── dtos
│       │   │   ├── mod.rs
│       │   │   ├── storage
│       │   │   │   ├── mod.rs
│       │   │   │   └── sql.rs <- Serialize/Deserialize SQLite Row to Prescription/PrescriptionEvent
│       │   │   └── transport
│       │   │       ├── http.rs <- Serialize/Deserialize JSON to Prescription/Command
│       │   │       └── mod.rs
│       │   └── mod.rs
│       └── mod.rs
└── main.rs <- Our Command Application

```
