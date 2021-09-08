# Test report for javascript / file:///tmp/top-repos-quality-repos-c6etyd31/be-nc-news.git HEAD 2d0d7d666013cfbc44da40e51266ea7f439066f0

### Classification report

PPCR: 0.909

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.864| 1.000| 0.993| 0.927| 0.924| 298| 300| 0.993 |
| `␣` | 1.000| 0.620| 0.429| 0.765| 0.600| 92| 133| 0.692 |
| `"` | 1.000| 0.941| 0.941| 0.970| 0.970| 34| 34| 1.000 |
| `⏎` | 0.895| 0.708| 0.630| 0.791| 0.739| 24| 27| 0.889 |
| `⏎␣⁺␣⁺` | 1.000| 0.684| 0.619| 0.813| 0.765| 19| 21| 0.905 |
| `⏎␣⁻␣⁻` | 1.000| 0.909| 0.909| 0.952| 0.952| 11| 11| 1.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `macro avg` | 0.823| 0.695| 0.646| 0.745| 0.707| 478| 526| 0.909 |
| `weighted avg` | 0.910| 0.893| 0.812| 0.888| 0.830| 478| 526| 0.909 |
| `micro avg` | 0.893| 0.893| 0.812| 0.893| 0.851| 478| 526| 0.909 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2 |298 |0 |0 |0 |0 |0 |0 |
|41 |34 |57 |1 |0 |0 |0 |0 |
|3 |7 |0 |17 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |2 |32 |0 |0 |
|2 |6 |0 |0 |0 |0 |13 |0 |
|0 |0 |0 |1 |0 |0 |0 |10 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9696969696969697, "precision": 1.0, "recall": 0.9411764705882353, "support": 34}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.7453259142614732, "precision": 0.8226435654353274, "recall": 0.6946252081027959, "support": 478}, "micro avg": {"f1-score": 0.893305439330544, "precision": 0.893305439330544, "recall": 0.893305439330544, "support": 478}, "weighted avg": {"f1-score": 0.8880065329924027, "precision": 0.9097836459440396, "recall": 0.893305439330544, "support": 478}, "\u2205": {"f1-score": 0.926905132192846, "precision": 0.863768115942029, "recall": 1.0, "support": 298}, "\u23ce": {"f1-score": 0.7906976744186046, "precision": 0.8947368421052632, "recall": 0.7083333333333334, "support": 24}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8125000000000001, "precision": 1.0, "recall": 0.6842105263157895, "support": 19}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9523809523809523, "precision": 1.0, "recall": 0.9090909090909091, "support": 11}, "\u2423": {"f1-score": 0.7651006711409396, "precision": 1.0, "recall": 0.6195652173913043, "support": 92}},
  "cl_report_full": {"\"": {"f1-score": 0.9696969696969697, "precision": 1.0, "recall": 0.9411764705882353, "support": 34}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.7071350352807728, "precision": 0.8226435654353274, "recall": 0.6458356271801649, "support": 526}, "micro avg": {"f1-score": 0.850597609561753, "precision": 0.893305439330544, "recall": 0.811787072243346, "support": 526}, "weighted avg": {"f1-score": 0.8297918917110476, "precision": 0.9168979648658759, "recall": 0.811787072243346, "support": 526}, "\u2205": {"f1-score": 0.9240310077519379, "precision": 0.863768115942029, "recall": 0.9933333333333333, "support": 300}, "\u23ce": {"f1-score": 0.7391304347826088, "precision": 0.8947368421052632, "recall": 0.6296296296296297, "support": 27}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7647058823529412, "precision": 1.0, "recall": 0.6190476190476191, "support": 21}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9523809523809523, "precision": 1.0, "recall": 0.9090909090909091, "support": 11}, "\u2423": {"f1-score": 0.6, "precision": 1.0, "recall": 0.42857142857142855, "support": 133}},
  "ppcr": 0.908745247148289
}
```
</details>