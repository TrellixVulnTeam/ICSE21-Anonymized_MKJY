# Test report for javascript / file:///tmp/top-repos-quality-repos-e3x98zh0/express HEAD b4eb1f59d39d801d7365c86b04500f16faeb0b1c

### Classification report

PPCR: 0.929

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.949| 0.993| 0.925| 0.971| 0.937| 7341| 7878| 0.932 |
| `␣` | 0.970| 0.931| 0.861| 0.950| 0.912| 3037| 3285| 0.925 |
| `'` | 0.996| 0.995| 0.995| 0.995| 0.995| 2074| 2074| 1.000 |
| `⏎` | 0.914| 0.805| 0.607| 0.856| 0.730| 611| 810| 0.754 |
| `⏎␣⁺␣⁺` | 0.974| 0.939| 0.914| 0.956| 0.943| 478| 491| 0.974 |
| `⏎⏎` | 0.823| 0.612| 0.508| 0.702| 0.628| 472| 569| 0.830 |
| `⏎␣⁻␣⁻` | 0.986| 0.970| 0.949| 0.978| 0.967| 440| 450| 0.978 |
| `weighted avg` | 0.957| 0.958| 0.890| 0.956| 0.919| 14453| 15557| 0.929 |
| `macro avg` | 0.945| 0.892| 0.823| 0.916| 0.873| 14453| 15557| 0.929 |
| `micro avg` | 0.958| 0.958| 0.890| 0.958| 0.922| 14453| 15557| 0.929 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|537 |7291 |39 |0 |8 |3 |0 |0 |
|248 |178 |2828 |9 |0 |4 |12 |6 |
|0 |8 |2 |2064 |0 |0 |0 |0 |
|199 |41 |23 |0 |492 |55 |0 |0 |
|97 |152 |2 |0 |29 |289 |0 |0 |
|13 |9 |11 |0 |9 |0 |449 |0 |
|10 |3 |10 |0 |0 |0 |0 |427 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9954183747287194, "precision": 0.9956584659913169, "recall": 0.9951783992285439, "support": 2074}, "macro avg": {"f1-score": 0.9156585989827993, "precision": 0.9446982027508062, "recall": 0.8924085666440117, "support": 14453}, "micro avg": {"f1-score": 0.9575866602089531, "precision": 0.9575866602089531, "recall": 0.9575866602089531, "support": 14453}, "weighted avg": {"f1-score": 0.9560831645065708, "precision": 0.9565873113187495, "recall": 0.9575866602089531, "support": 14453}, "\u2205": {"f1-score": 0.9706450109831591, "precision": 0.9491017964071856, "recall": 0.9931889388366707, "support": 7341}, "\u23ce": {"f1-score": 0.856396866840731, "precision": 0.9144981412639405, "recall": 0.8052373158756138, "support": 611}, "\u23ce\u23ce": {"f1-score": 0.7023086269744836, "precision": 0.8233618233618234, "recall": 0.6122881355932204, "support": 472}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9563365282215123, "precision": 0.9739696312364425, "recall": 0.9393305439330544, "support": 478}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9782359679266897, "precision": 0.9861431870669746, "recall": 0.9704545454545455, "support": 440}, "\u2423": {"f1-score": 0.9502688172043011, "precision": 0.9701543739279588, "recall": 0.931182087586434, "support": 3037}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9954183747287194, "precision": 0.9956584659913169, "recall": 0.9951783992285439, "support": 2074}, "macro avg": {"f1-score": 0.8733555562286369, "precision": 0.9446982027508062, "recall": 0.8228878707973962, "support": 15557}, "micro avg": {"f1-score": 0.9223592135954681, "precision": 0.9575866602089531, "recall": 0.8896316770585588, "support": 15557}, "weighted avg": {"f1-score": 0.9186367685228695, "precision": 0.9552096563572057, "recall": 0.8896316770585588, "support": 15557}, "\u2205": {"f1-score": 0.937146529562982, "precision": 0.9491017964071856, "recall": 0.9254887027164255, "support": 7878}, "\u23ce": {"f1-score": 0.7299703264094957, "precision": 0.9144981412639405, "recall": 0.6074074074074074, "support": 810}, "\u23ce\u23ce": {"f1-score": 0.6282608695652174, "precision": 0.8233618233618234, "recall": 0.507908611599297, "support": 569}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9432773109243697, "precision": 0.9739696312364425, "recall": 0.9144602851323829, "support": 491}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9671574178935447, "precision": 0.9861431870669746, "recall": 0.9488888888888889, "support": 450}, "\u2423": {"f1-score": 0.9122580645161291, "precision": 0.9701543739279588, "recall": 0.860882800608828, "support": 3285}},
  "ppcr": 0.9290351610207623
}
```
</details>