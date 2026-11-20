  export foreign-alias-guard-observations
  -- | Test a guarded rewrite through its type, effect and free-local invariants.
  foreign-alias-probe-accept :: ForeignPlan -> CoreTerm -> Bool
  foreign-alias-probe-accept plan term = case fuse-scoped-foreign-plan "m.Probe.call" plan term of
    Some rewritten -> not (term-uses 50 rewritten) && scheme-equivalent (mono (core-type term)) (mono (core-type rewritten))
      && same-shape (core-effects term) (core-effects rewritten)
    None -> False
  -- | Require a refused shape to keep its original foreign function-value path.
  foreign-alias-probe-refuse :: ForeignPlan -> CoreTerm -> Bool
  foreign-alias-probe-refuse plan term = case fuse-scoped-foreign-plan "m.Probe.call" plan term of Some rewritten -> False; None -> True
  -- | Build a lexical alias with its actual body result type and stable local identity.
  foreign-alias-probe-let :: String -> Scheme -> CoreTerm -> CoreTerm -> CoreTerm
  foreign-alias-probe-let name scheme value body = CLet (Span 0 0 1 1) (core-type body) 50 name scheme value body
  -- | Exercise private guard boundaries without adding a production compiler API.
  foreign-alias-guard-observations :: List Bool
  foreign-alias-guard-observations =
    let sp = Span 0 0 1 1 in
    let byte = UCon "U8" KType in let word = UCon "U64" KType in
    let effects = URow (UArg (UCon "Global" KEffect) UEnd) in
    let function = UFunEff byte effects word in let scheme = mono function in
    let value = CUse sp function "m.Probe.call" "foreign" Nil in
    let local = CLocal sp function 50 "#argument-function" Nil in
    let argument = CInt sp byte "1" in
    let call = CApplyEff sp word effects local argument in
    let valid = foreign-alias-probe-let "#argument-function" scheme value call in
    let two-function = UFun byte function in let two-scheme = mono two-function in
    case (foreign-plan 64 Nil "probe" scheme, foreign-plan 64 Nil "probe_two" two-scheme) of
      (Ok plan, Ok two-plan) ->
        let saved = CLet sp word 60 "saved" (mono byte) argument call in
        let repeated = CLet sp word 60 "first" (mono word) call call in
        let branch = CIf sp word (CUse sp (UCon "Bool" KType) "b.True" "constructor" Nil) call call in
        let shadow = CLet sp word 50 "shadow" scheme value call in
        let escaped = CApply sp word (CUse sp (UFun function word) "m.Probe.consume" "function" Nil) local in
        let dependent = UPi 80 "byte" byte effects word in
        let dependent-value = CUse sp dependent "m.Probe.call" "foreign" Nil in
        let dependent-call = CApplyEff sp word effects (CLocal sp dependent 50 "#argument-function" Nil) argument in
        let two-value = CUse sp two-function "m.Probe.call" "foreign" Nil in
        let partial = CApply sp function two-value argument in
        let partial-use = CApply sp function (CLocal sp two-function 50 "#argument-function" Nil) argument in
        [ foreign-alias-probe-accept plan valid
        , foreign-alias-probe-accept plan (foreign-alias-probe-let "#argument-function" scheme value saved)
        , foreign-alias-probe-refuse plan (foreign-alias-probe-let "ordinary" scheme value call)
        , foreign-alias-probe-refuse plan (foreign-alias-probe-let "#argument-function" scheme value local)
        , foreign-alias-probe-refuse plan (foreign-alias-probe-let "#argument-function" scheme value repeated)
        , foreign-alias-probe-refuse plan (foreign-alias-probe-let "#argument-function" scheme value branch)
        , foreign-alias-probe-refuse plan (foreign-alias-probe-let "#argument-function" scheme value shadow)
        , foreign-alias-probe-refuse plan (foreign-alias-probe-let "#argument-function" scheme value escaped)
        , foreign-alias-probe-refuse plan (foreign-alias-probe-let "#argument-function" scheme value (CApplyEff sp word effects local local))
        , foreign-alias-probe-refuse plan (foreign-alias-probe-let "#argument-function" scheme value (CApply sp word call argument))
        , foreign-alias-probe-refuse plan (foreign-alias-probe-let "#argument-function" scheme (CForget sp function value) call)
        , foreign-alias-probe-refuse plan (foreign-alias-probe-let "#argument-function" scheme (CUse sp function "m.Probe.call" "function" Nil) call)
        , foreign-alias-probe-refuse plan (foreign-alias-probe-let "#argument-function" scheme (CUse sp function "m.Probe.call" "foreign" [byte]) call)
        , foreign-alias-probe-refuse plan (foreign-alias-probe-let "#argument-function" (MkScheme [MkQuant 70 "r" KRegion False] function) value call)
        , foreign-alias-probe-refuse plan (foreign-alias-probe-let "#argument-function" (qualified Nil [MkRequirement "hidden" (UCon "Unit" KType)] function) value call)
        , foreign-alias-probe-refuse plan (foreign-alias-probe-let "#argument-function" (mono (UFun byte word)) value call)
        , foreign-alias-probe-refuse plan (foreign-alias-probe-let "#argument-function" (mono dependent) dependent-value dependent-call)
        , foreign-alias-probe-refuse two-plan (foreign-alias-probe-let "#argument-function" scheme partial call)
        , foreign-alias-probe-refuse two-plan (foreign-alias-probe-let "#argument-function" two-scheme two-value partial-use)
        , (case fuse-scoped-foreign-plan "m.Probe.other" plan valid of None -> True; Some rewritten -> False)
        ]
      other -> [False]
