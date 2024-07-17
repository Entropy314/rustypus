



```python
    self.default_variator = {Real : GAOperator(SBX(), PM()),
                            Binary : GAOperator(HUX(), BitFlip()),
                            Permutation : CompoundOperator(PMX(), Insertion(), Swap()),
                            Subset : GAOperator(SSX(), Replace())}

    self.default_mutator = {Real : PM(),
                            Binary : BitFlip(),
                            Permutation : CompoundMutation(Insertion(), Swap()),
                            Subset : Replace()}
```