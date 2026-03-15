# OMT-1: Just getting started

## Objective

Create a repeatable path to instantiate **20 apps** in Linear and/or directly in this repository with consistent standards, ownership, and delivery pace.

## Success criteria

- 20 apps are scaffolded and tracked with clear ownership.
- Every app meets a shared baseline (build, test, deploy, observability).
- Most setup steps are automated to reduce manual drift.

## Phase plan

### Phase 1: Foundation (Week 1)

- Define supported app archetypes (API, worker, frontend, internal tool).
- Publish a baseline "Definition of Done" for every new app:
  - health check
  - lint and tests in CI
  - deployment config
  - logging and metrics
  - owner and runbook
- Establish Linear labels and workflow states for app rollout.

### Phase 2: Templates and automation (Week 1-2)

- Add or refine a reusable app template in-repo.
- Build a `create-app` workflow (script or CLI) to scaffold:
  - project structure
  - CI config
  - environment variable examples
  - metadata placeholders (owner, tier, service name)
- Add a companion issue template for Linear so app tickets are consistent.

### Phase 3: Pilot batch (Apps 1-5, Week 2-3)

- Stand up 5 representative apps across different archetypes.
- Track setup time and manual interventions required per app.
- Fold pilot learnings back into templates and automation.

### Phase 4: Scale batches (Apps 6-20, Week 3-6)

- Deliver three batches of 5 apps each.
- Freeze template changes during an active batch except for blockers.
- Use dependency links in Linear to surface blockers early.

### Phase 5: Governance and quality (Ongoing)

- Enforce baseline checks for all apps:
  - CI green
  - deployment validation
  - observability present
  - owner assigned
- Run periodic drift checks against template expectations.

### Phase 6: Operational readiness (Week 6+)

- Confirm runbooks, escalation paths, and maintenance status per app.
- Publish a dashboard view for health and deployment posture.
- Mark each app as active, maintenance, or deprecated.

## Suggested Linear structure

Create one parent initiative: **20-App Factory Rollout**.

Recommended epics:

1. Platform Foundation
2. Template and Scaffolding
3. Pilot Batch (1-5)
4. Scale Batch A (6-10)
5. Scale Batch B (11-15)
6. Scale Batch C (16-20)
7. Operational Readiness

For each app issue, include:

- App name
- Archetype
- Owner
- Repository path
- Definition of Done checklist
- Dependencies
- Target date

## KPIs to track weekly

- Apps instantiated per week
- Median time from issue creation to app-ready state
- Percentage of apps passing full CI
- Percentage of setup steps automated
- Post-instantiation defect count

## Top risks and mitigations

1. **Template instability**: run pilot first and batch releases.
2. **Manual drift**: automate app creation and periodic drift checks.
3. **Ownership gaps**: require owner + runbook before completion.
4. **CI bottlenecks**: standardize pipelines and caching.
5. **Scope creep**: document exception policy for customizations.

## 30/60/90-day targets

- **Day 30**: Template stable; first 5 apps complete.
- **Day 60**: 15 apps instantiated with quality gates.
- **Day 90**: 20 apps running with clear ownership and support posture.
