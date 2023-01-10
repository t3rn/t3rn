---
category: **Story**
title: _here goes your title_
---
### Epic
should be part of an epic

### Scope
> file scope of changes + edit modificator MOD/ADD/DEL

> > _example_
> > - MOD pallet circuit::square_up
> > - MOD pallet account manager::(deposit + finalize)

### IO
#### Input
> describe input data: Structures and Actors
> > _example_
> > - Array SFX[]
> > - Actor Requester

#### Output
> describe output data: Structures and Actors
> > _example_
> > - Claimable Artifacts (AccountManager)


### Goal
> describe the goal of the story, especially the business value and how input translates to output
> > _example_
> > Handle all submissions in fallible manner:
> > - Validate whether requester affords all SFX[] in given asset_id
> > - Validate whether bidder affords all Bid[] requirements (min_bid amount + insurance + bonded deposit to co-executors) in given asset_id

### Acceptance Criteria
> describe the acceptance criteria for the story to be considered done
> > _example_
> > - fallible checks for SFX[] and Bid[] submissions
> > - infallible checks for Xtx finalizations incorporated into CircuitMachine::apply

### Logs
> maintain logs for each working session (probably commit messages of the main changes are enough). Note additional problems that occurred and incline solutions. Leave Closing Notes before the following session to ease the pick-up process.
> > _example_
> > #### Session 1:
> > Covered SquareUp::try_request with negative and positive tests using AM::can_afford and AM::deposit
> > - Problem: AM::deposit doesn't take into account chained deposits for SFX - introduce additional AM::deposit_batch method which ensure all the deposits can go through at once (or none of them)
> > ##### Closing notes: Add AM::deposit_batch method which ensures all the deposits can go through at once (or none of them) and continue testing try_request. Proceed with try_bid and try_execute.
